//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2585/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2585(t19047: f64, t4997: f64, t19040: f64, t5005: f64, t71095: f64, t71097: f64, t71106: f64, t71109: f64, t71112: f64, t71114: f64, t71118: f64, t71217: f64, t71221: f64, t71225: f64, t71227: f64, t71230: f64, t71233: f64, t71236: f64, t71238: f64, t71241: f64, t71245: f64, t71247: f64, t71249: f64, t71251: f64) -> (f64, f64, f64) {
    let t72181 = t19047 * t4997;
    let t72183 = t5005 * t19040;
    let t72195 = t71095 - t71097 + t71106 - t71109 - t71112 + t71114 + t71118 - t71217 + t71221 - t71225 + t71227 + t71230 - t71233 - t71236 + t71238 - t71241 + t71245 - t71247 - t71249 - t71251;
    (t72181, t72183, t72195)
}
