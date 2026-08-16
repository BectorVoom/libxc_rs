//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 356/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk356(t1742: f64, t479: f64, t471: f64, t1230: f64, t1653: f64, t248: f64, t1174: f64, t1195: f64, t1213: f64, t1224: f64, t1227: f64, t1706: f64, t1726: f64, t1731: f64, t1737: f64, t467: f64, t488: f64) -> (f64, f64, f64, f64) {
    let t1743 = t479 * t1742;
    let t1744 = t471 * t1743;
    let t1748 = t248 * t1230 * t1653;
    let t1751 = -t1706 * t467 / 36.0_f64 + t1195 - t1174 * t1726 / 288.0_f64 + t1731 * t488 / 3072.0_f64 + t1213 * t1737 / 3072.0_f64 - t1744 * t488 / 576.0_f64 + t1224 - t1227 * t1748 / 4608.0_f64;
    (t1743, t1744, t1748, t1751)
}
