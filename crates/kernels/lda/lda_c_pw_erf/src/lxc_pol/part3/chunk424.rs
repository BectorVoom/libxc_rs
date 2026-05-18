//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 424/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk424<F: Float>(t598: F, t611: F, t925: F, t933: F, t7: F) -> (F, F, F, F) {
    let t1615 = t598 * t611;
    let t1619 = -F::new(0.55) * t925 + F::new(5.0) / F::new(18.0) * t933;
    let t1620 = t1619 * M_PI;
    let t1621 = t1620 * t7;
    (t1615, t1619, t1620, t1621)
}
