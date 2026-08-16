//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1363/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1363(t71970: f64, t72026: f64, t72129: f64, t72170: f64, t823: f64, t20526: f64, t69855: f64, t198: f64, t6368: f64, t1692: f64, t1812: f64, t18728: f64, t18807: f64, t19819: f64, t19821: f64, t19825: f64, t20417: f64, t20510: f64, t20514: f64, t21356: f64, t21659: f64, t2439: f64, t30: f64, t3552: f64, t5539: f64, t6120: f64, t69800: f64, t69838: f64, t69864: f64, t70241: f64, t70244: f64, t70290: f64) -> (f64, f64, f64, f64, f64) {
    let t72172 = t71970 + t72026 + t72129 + t72170;
    let t72173 = t72172 * t823;
    let t72187 = 2.0_f64 * t20526 * t69855;
    let t72188 = t198 * t6368;
    let t72203 = 3.0_f64 * t3552 * t1812 * t69838 + t1692 * t72173 * t30 / 2.0_f64 - t1692 * t20514 * t19825 + 3.0_f64 * t2439 * t20510 * t6120 - 3.0_f64 * t18728 * t70290 - 3.0_f64 / 2.0_f64 * t18728 * t69864 - t72187 + 2.0_f64 * t72188 * t19819 - t1692 * t20514 * t19821 - t1692 * t18807 * t21356 - 3.0_f64 * t20526 * t70244 + t20526 * t70241 - 6.0_f64 * t20417 * t69800 + 3.0_f64 / 2.0_f64 * t2439 * t21659 * t5539;
    (t72172, t72173, t72187, t72188, t72203)
}
