//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 705/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk705<F: Float>(t571: F, t6287: F, t2002: F, t4763: F, t2392: F, t3859: F, t1325: F, t2123: F, t822: F, t2527: F, t514: F, t211: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6289 = F::new(4.0) / F::new(45.0) * t571 * t6287;
    let t6291 = F::new(16.0) / F::new(45.0) * t4763 * t2002;
    let t6292 = t3859 * t2392;
    let t6293 = t1325 * t6292;
    let t6294 = F::new(32.0) / F::new(135.0) * t6293;
    let t6295 = t822 * t2123;
    let t6296 = F::new(8.0) / F::new(45.0) * t6295;
    let t6297 = t514 * t2527;
    let t6298 = t211 * t6297;
    (t6289, t6291, t6292, t6293, t6294, t6295, t6296, t6297, t6298)
}
