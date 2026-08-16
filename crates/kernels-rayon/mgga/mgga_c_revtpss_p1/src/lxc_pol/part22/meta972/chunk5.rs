//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3257/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3257(t125: f64, t18615: f64, t10744: f64, t18418: f64, t808: f64, t10900: f64, t18627: f64, t2394: f64, t2724: f64, t2747: f64, t4362: f64, t4364: f64, t4366: f64, t50573: f64, t50577: f64, t50579: f64, t50581: f64, t50586: f64, t50590: f64, t50594: f64, t50598: f64, t5984: f64, t800: f64) -> (f64, f64) {
    let t61791 = t125 * t18615;
    let t61797 = t10744 * t808 * t18418;
    let t61814 = -0.15246000842785598467e-3_f64 * t50573 + 0.85748036236139473944e-3_f64 * t4362 * t4364 * t61791 * t4366 + 0.25410001404642664112e-5_f64 * t61797 - t10900 * t800 * t5984 * t2394 / 4.0_f64 - 0.57165357490759649296e-4_f64 * t50577 + 0.20007875121765877254e-2_f64 * t50579 + 0.54208002996571016772e-3_f64 * t50581 - 0.11433071498151929859e-3_f64 * t50586 - 0.57165357490759649296e-4_f64 * t50590 - 0.17149607247227894789e-2_f64 * t4362 * t2747 * t18627 * t2724 + 0.10164000561857065645e-3_f64 * t50594 + 0.15246000842785598467e-3_f64 * t50598;
    (t61791, t61814)
}
