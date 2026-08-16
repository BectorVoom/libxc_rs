//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1401/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1401(t10510: f64, t11044: f64, t675: f64, t886: f64, t10995: f64, t268: f64, t2828: f64, t252: f64, t257: f64, t39644: f64, t8779: f64, t123: f64, t2434: f64, t2771: f64) -> (f64, f64, f64, f64) {
    let t41038 = t11044 * t10510;
    let t41040 = t675 * t886;
    let t41043 = t10995 * t268 * t41040 * t2828;
    let t41049 = 0.11638313500518478545e-4_f64 * t39644 * t252 * t257 * t8779 * t268;
    let t41052 = t10995 * t123 * t2434 * t2771;
    (t41038, t41043, t41049, t41052)
}
