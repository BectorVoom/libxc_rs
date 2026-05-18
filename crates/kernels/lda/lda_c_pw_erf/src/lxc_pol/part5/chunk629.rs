//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 629/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk629<F: Float>(t2001: F, t3854: F, t1318: F, t1519: F, t795: F, t2123: F, t565: F, t790: F, t925: F, t1968: F, t325: F, t1973: F) -> (F, F, F, F, F, F, F) {
    let t4581 = t3854 * t2001;
    let t4583 = F::new(32.0) / F::new(135.0) * t1318 * t4581;
    let t4592 = t795 * t1519;
    let t4595 = F::new(8.0) / F::new(45.0) * t565 * t2123;
    let t4600 = t925 * t790;
    let t4602 = t325 * t1968;
    let t4604 = t325 * t1973;
    (t4581, t4583, t4592, t4595, t4600, t4602, t4604)
}
