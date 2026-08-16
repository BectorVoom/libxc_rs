//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 757/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk757(t2132: f64, t2183: f64, t296: f64, t297: f64, t306: f64, t307: f64, t6101: f64, t1275: f64, t815: f64, t817: f64, t312: f64, t317: f64, t6100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6583 = t2183 * t2132;
    let t6621 = 1.0_f64 / t297 / t296;
    let t6635 = 1.0_f64 / t307 / t306;
    let t6648 = 154.0_f64 / 27.0_f64 * t6101;
    let t6654 = t815 * t1275;
    let t6659 = t817 * t817;
    let t6660 = 1.0_f64 / t6659;
    let t6661 = t312 * t6660;
    let t6678 = 154.0_f64 / 27.0_f64 * t317 * t6100;
    (t6583, t6621, t6635, t6648, t6654, t6659, t6660, t6661, t6678)
}
