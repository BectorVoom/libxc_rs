//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2030/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2030(t102558: f64, t102580: f64, t102597: f64, t102614: f64, t102629: f64, t102765: f64, t102790: f64, t102822: f64, t1375: f64, t1378: f64, t27068: f64, t27115: f64, t29372: f64, t3758: f64, t5215: f64, t5354: f64, t84423: f64, t90706: f64, t93461: f64, t93467: f64, t97529: f64, t97537: f64, t97548: f64) -> f64 {
    let t102828 = 0.15352717957250113407e0_f64 * t97529 + t84423 - 2.0_f64 * t5215 * t27115 + t93461 + 0.76763589786250567037e-1_f64 * t97537 + t90706 + t93467 - 0.76763589786250567037e-1_f64 * t97548 - 2.0_f64 * t27068 * t5354 + 2.0_f64 * t3758 * t29372 - t1375 * t1378 * (t102558 + t102580 + t102597 + t102614 + t102629 + t102765 + t102790 + t102822);
    t102828
}
