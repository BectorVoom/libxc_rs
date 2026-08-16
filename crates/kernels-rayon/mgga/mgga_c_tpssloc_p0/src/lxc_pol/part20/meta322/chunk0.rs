//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1592/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1592(t11620: f64, t1246: f64, t1235: f64, t3507: f64, t3625: f64, t1155: f64, t3375: f64, t3396: f64, t1164: f64, t11128: f64, t11133: f64, t11179: f64, t11182: f64, t11184: f64, t11187: f64, t11405: f64, t11409: f64, t11426: f64, t11429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11621 = t11620 * t1246;
    let t11624 = t1235 * t3507;
    let t11625 = t11624 * t3625;
    let t11628 = t3375 * t1155;
    let t11629 = t11628 * t3396;
    let t11631 = 0.35089341735807877242e1_f64 * t1164 * t11629;
    let t11632 = -t11426 + t11429 - t11405 + t11409 + t11631 - t11128 - t11133 + t11179 + t11182 + t11184 + t11187;
    (t11621, t11624, t11625, t11629, t11631, t11632)
}
