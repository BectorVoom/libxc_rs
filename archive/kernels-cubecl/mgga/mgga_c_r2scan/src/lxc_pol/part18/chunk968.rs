//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 968/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk968<F: Float>(t11629: F, t3275: F, t3277: F, t10918: F, t2867: F, t11479: F, t3262: F, t3264: F, t3332: F, t7629: F, t7628: F, t8156: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11631 = t3275 * t11629 * t3277;
    let t11632 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11631;
    let t11634 = t3275 * t10918 * t2867;
    let t11635 = t11634 / F::cast_from(4.0_f64);
    let t11637 = t3262 * t11479 * t3264;
    let t11638 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t11637;
    let t11640 = t3332 * t7629;
    let t11641 = t7628 * t11640;
    let t11643 = t3332 * t8156;
    (t11631, t11632, t11634, t11635, t11637, t11638, t11640, t11641, t11643)
}
