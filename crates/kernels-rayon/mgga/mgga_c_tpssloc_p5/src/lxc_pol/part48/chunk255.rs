//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 255/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk255(t1215: f64, t475: f64, t1214: f64, t248: f64, t122: f64, t374: f64, t486: f64, t485: f64, t372: f64, t483: f64, t479: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1216 = t1215 * t475;
    let t1218 = t248 * t1214 * t1216;
    let t1222 = t374 * t122 * t486;
    let t1224 = t485 * t1222 / 4608.0_f64;
    let t1225 = t483 * t372;
    let t1226 = t479 * t1225;
    let t1227 = t471 * t1226;
    (t1216, t1218, t1222, t1224, t1225, t1226, t1227)
}
