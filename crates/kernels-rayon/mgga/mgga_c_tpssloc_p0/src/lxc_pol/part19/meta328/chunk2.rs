//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1171/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1171(t12126: f64, t588: f64, t39037: f64, t522: f64, t2221: f64, t3826: f64, t3824: f64, t12132: f64, t592: f64, t3696: f64, t2223: f64, t39844: f64, t39846: f64, t39852: f64, t39854: f64, t39856: f64, t39858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40221 = t588 * t12126;
    let t40222 = 48.0_f64 * t40221;
    let t40224 = 840.0_f64 * t39037 * t522;
    let t40225 = t2221 * t3826;
    let t40226 = 144.0_f64 * t40225;
    let t40227 = t2221 * t3824;
    let t40228 = 72.0_f64 * t40227;
    let t40230 = 16.0_f64 * t592 * t12132;
    let t40231 = t2221 * t3696;
    let t40232 = 72.0_f64 * t40231;
    let t40233 = t2223 * t3696;
    let t40234 = 192.0_f64 * t40233;
    let t40235 = t39844 + t39846 - t39852 + t39854 + t39856 - t39858 + t40222 + t40224 + t40226 + t40228 - t40230 + t40232 - t40234;
    (t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40235)
}
