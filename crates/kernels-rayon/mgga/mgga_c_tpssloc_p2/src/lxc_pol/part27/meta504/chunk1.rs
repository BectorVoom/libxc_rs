//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1899/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1899(t25261: f64, t829: f64, t1510: f64, t22992: f64, t13380: f64, t232: f64, t6646: f64, t1888: f64, t1499: f64, t23002: f64, t23014: f64, t23026: f64, t23028: f64, t23032: f64, t23166: f64, t23169: f64, t23174: f64, t25239: f64, t25243: f64, t25246: f64, t25252: f64, t25256: f64, t25259: f64, t2617: f64, t4291: f64, t6660: f64, t7533: f64, t812: f64) -> (f64, f64, f64, f64, f64) {
    let t25262 = t25261 * t829;
    let t25269 = t22992 * t1510;
    let t25272 = t13380 * t232;
    let t25273 = t6646 * t25272;
    let t25274 = t1888 * t25273;
    let t25276 = -0.82246703342411321825e-2_f64 * t25239 - 0.82246703342411321825e-2_f64 * t25243 + 0.41123351671205660912e-2_f64 * t25246 + 0.49348022005446793095e-1_f64 * t25252 + 0.19190897446562641759e-1_f64 * t23002 - t812 * t25256 - 0.41123351671205660912e-2_f64 * t25259 - t4291 * t25262 + t23014 - 0.41123351671205660912e-2_f64 * t23026 - 0.19190897446562641759e-1_f64 * t23028 + t23032 + 0.82246703342411321824e-2_f64 * t23166 + 0.38381794893125283518e-1_f64 * t23169 - t2617 * t7533 - t812 * t25269 - t23174 + t1499 * t6660 - 0.82246703342411321825e-2_f64 * t25274;
    (t25262, t25269, t25272, t25273, t25276)
}
