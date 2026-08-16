//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1123/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1123(t2299: f64, t7780: f64, t3196: f64, t33953: f64, t13364: f64, t31195: f64, t31231: f64, t7637: f64, t8545: f64, t31212: f64, t31222: f64, t31224: f64, t31227: f64, t31230: f64, t31237: f64, t31239: f64, t31241: f64, t31245: f64, t31247: f64, t35404: f64, t35408: f64, t35411: f64, t35415: f64) -> (f64, f64) {
    let t35418 = t7780 * t2299;
    let t35420 = t33953 * t3196;
    let t35422 = t31195 * t13364 * t35420;
    let t35424 = 0.34299214494455789578e-2_f64 * t31231;
    let t35425 = t7637 * t8545;
    let t35432 = t35404 + 0.28303283060643736861e-1_f64 * t31212 - t35408 - t35411 - 0.42874018118069736972e-3_f64 * t31222 - t35415 / 32.0_f64 - 0.4501771902397322382e-1_f64 * t31224 + t31227 + 0.33020496904084359671e-1_f64 * t35418 + 0.10718504529517434243e-2_f64 * t35422 + t31230 + t35424 + 0.47637797908966374413e-2_f64 * t35425 - 0.31448092289604152068e-3_f64 * t31237 - 0.31448092289604152068e-3_f64 * t31239 - 0.83861579438944405513e-3_f64 * t31241 + 0.31448092289604152068e-3_f64 * t31245 + 0.32155513588552302729e-2_f64 * t31247;
    (t35420, t35432)
}
