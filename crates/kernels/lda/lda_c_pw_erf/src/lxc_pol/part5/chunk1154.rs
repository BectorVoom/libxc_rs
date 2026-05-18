//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1154/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1154<F: Float>(t3832: F, t571: F, t593: F, t7414: F, t1472: F, t7716: F, t16305: F, t743: F, t2017: F, t34: F, t6365: F, t4868: F) -> (F, F, F, F, F, F) {
    let t21204 = F::new(8.0) / F::new(9.0) * t571 * t3832 * t7414 * t593;
    let t21206 = F::new(4.0) / F::new(9.0) * t1472 * t7716;
    let t21207 = t16305 * t743;
    let t21210 = F::new(4.0) / F::new(9.0) * t571 * t2017 * t21207;
    let t21211 = t6365 * t34;
    let t21214 = F::new(8.0) / F::new(9.0) * t571 * t4868 * t21211;
    (t21204, t21206, t21207, t21210, t21211, t21214)
}
