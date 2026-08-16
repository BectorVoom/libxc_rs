//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1192/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1192(t1674: f64, t1713: f64, t8034: f64, t10040: f64, t1427: f64, t1680: f64, t2166: f64, t24753: f64, t24893: f64, t32278: f64, t36684: f64, t36686: f64, t36689: f64, t38559: f64, t38563: f64, t5645: f64, t567: f64, t7297: f64, t8040: f64, t8372: f64, t9448: f64, t9460: f64, t9469: f64) -> f64 {
    let t40992 = t1674 * t8034 * t1713;
    let t41000 = -t10040 * t2166 * t567 + 12.0_f64 * t1427 * t36686 * t8372 - 2.0_f64 * t1680 * t567 * t9448 - 3.0_f64 * t24753 * t7297 * t8040 - 6.0_f64 * t24893 * t8040 * t8372 + 6.0_f64 * t32278 * t567 * t9469 + 12.0_f64 * t38559 * t7297 * t9460 - 6.0_f64 * t38563 * t7297 * t8040 + 12.0_f64 * t5645 * t8034 * t8372 - t36684 + t36689 + 6.0_f64 * t40992;
    t41000
}
