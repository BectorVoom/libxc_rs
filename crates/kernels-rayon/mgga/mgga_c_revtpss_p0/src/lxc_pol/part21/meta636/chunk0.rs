//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2408/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2408(t11029: f64, t9303: f64, t39501: f64, t781: f64, t10510: f64, t11044: f64, t675: f64, t886: f64, t10995: f64, t268: f64, t2828: f64, t252: f64, t257: f64, t39644: f64, t8779: f64) -> (f64, f64, f64, f64, f64) {
    let t41034 = t9303 * t11029;
    let t41037 = 0.56911289235245161963e-1_f64 * t39501 * t781;
    let t41038 = t11044 * t10510;
    let t41040 = t675 * t886;
    let t41043 = t10995 * t268 * t41040 * t2828;
    let t41049 = 0.11638313500518478545e-4_f64 * t39644 * t252 * t257 * t8779 * t268;
    (t41034, t41037, t41038, t41043, t41049)
}
