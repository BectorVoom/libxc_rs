//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1185/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1185(t12092: f64, t12095: f64, t12100: f64, t12103: f64, t12109: f64, t12111: f64, t12200: f64, t12204: f64, t11331: f64, t11335: f64, t11340: f64, t11344: f64, t11347: f64, t11350: f64, t11352: f64, t11354: f64) -> f64 {
    let t41138 = t12092 / 2.0_f64;
    let t41139 = 15.0_f64 / 8.0_f64 * t12095;
    let t41140 = 5.0_f64 / 8.0_f64 * t12100;
    let t41141 = 5.0_f64 / 8.0_f64 * t12103;
    let t41142 = 3.0_f64 / 2.0_f64 * t12109;
    let t41143 = t12111 / 2.0_f64;
    let t41144 = t12200 / 2.0_f64;
    let t41145 = 5.0_f64 / 8.0_f64 * t12204;
    let t41146 = -t41138 - t41139 - t41140 - t41141 + t11331 + t11335 - t11340 + t11344 + t11347 + t11350 + t11352 + t41142 + t41143 + t41144 - t41145 + t11354;
    t41146
}
