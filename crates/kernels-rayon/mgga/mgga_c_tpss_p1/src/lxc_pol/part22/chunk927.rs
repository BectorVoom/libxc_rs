//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 927/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk927(t8660: f64, t2480: f64, t841: f64, t2617: f64, t894: f64, t2620: f64, t317: f64, t314: f64, t8664: f64, t2586: f64, t294: f64, t2613: f64, t2621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8723 = 0.55403703703703703703e-1_f64 * t8660;
    let t8737 = t841 * t2480;
    let t8749 = 1.0_f64 / t2617 / t894;
    let t8752 = 1.0_f64 / t2620 / t317;
    let t8756 = 0.28842592592592592592e-1_f64 * t8660;
    let t8772 = 1.0_f64 / t2617 / t314;
    let t8796 = 0.93932222222222222223e0_f64 * t8660;
    let t8797 = 0.36793333333333333333e0_f64 * t8664;
    let t8812 = t294 * t2586;
    let t8833 = t2613 * t2621;
    (t8723, t8737, t8749, t8752, t8756, t8772, t8796, t8797, t8812, t8833)
}
