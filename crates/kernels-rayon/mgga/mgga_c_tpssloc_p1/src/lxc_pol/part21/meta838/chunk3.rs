//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2993/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2993(t10231: f64, t17157: f64, t973: f64, t17161: f64, t17183: f64, t2970: f64, t17178: f64, t17599: f64, t17602: f64, t17994: f64, t2960: f64, t43228: f64, t50242: f64, t50250: f64, t50255: f64, t50258: f64, t50262: f64, t59730: f64, t59746: f64, t977: f64) -> f64 {
    let t62657 = t973 * t10231 * t17157;
    let t62660 = t973 * t10231 * t17161;
    let t62663 = t973 * t2970 * t17183;
    let t62666 = t973 * t10231 * t17178;
    let t62680 = 4.0_f64 / 27.0_f64 * t2960 * t17994 - 4.0_f64 / 81.0_f64 * t2960 * t17599 - 14.0_f64 / 243.0_f64 * t2960 * t17602 - t62657 / 54.0_f64 + t62660 / 162.0_f64 - t62663 / 216.0_f64 + t62666 / 324.0_f64 - t973 * t977 * t59730 / 72.0_f64 + t43228 / 1296.0_f64 - t50242 / 108.0_f64 - t50250 / 216.0_f64 + t50255 / 384.0_f64 + t50258 / 3456.0_f64 - t50262 / 5184.0_f64 + t973 * t977 * t59746 / 48.0_f64;
    t62680
}
