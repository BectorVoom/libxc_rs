//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1317/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1317(t10987: f64, t135: f64, t973: f64, t10394: f64, t10405: f64, t10408: f64, t10415: f64, t10937: f64, t10944: f64, t10957: f64, t10988: f64, t2771: f64, t2780: f64, t2960: f64, t3064: f64, t3070: f64, t3071: f64, t3073: f64, t3121: f64, t3134: f64, t42505: f64, t42508: f64, t42511: f64, t42514: f64, t42518: f64, t42522: f64) -> f64 {
    let t42530 = t973 * t135 * t10987;
    let t42540 = -t10937 * t10394 / 72.0_f64 - t42505 * t10405 / 36.0_f64 + t42508 * t10415 / 72.0_f64 + t42511 * t3073 / 384.0_f64 - t42514 / 108.0_f64 + 95.0_f64 / 1296.0_f64 * t10957 * t3064 - 5.0_f64 / 324.0_f64 * t42518 + 19.0_f64 / 144.0_f64 * t42522 * t3134 - t2960 * t10988 / 27.0_f64 - 28.0_f64 / 243.0_f64 * t2960 * t10944 + t42530 / 216.0_f64 + t3070 * t3071 * t3121 * t2780 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t3070 * t10408 * t3121 * t2771;
    t42540
}
