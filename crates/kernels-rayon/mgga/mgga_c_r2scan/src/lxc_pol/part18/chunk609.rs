//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 609/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk609(t322: f64, t3366: f64, t819: f64, t3357: f64, t3359: f64, t3361: f64, t3364: f64) -> (f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t3367 = t819 * t3366;
    let t3368 = t3367 / 3.0_f64;
    let t3369 = t3357 + t3359 / 8.0_f64 - t3361 / 8.0_f64 + t3364 / 4.0_f64 + t3368;
    let t3370 = piecewise3(t324, 0.0_f64, t3369);
    (t3368, t3369, t3370)
}
