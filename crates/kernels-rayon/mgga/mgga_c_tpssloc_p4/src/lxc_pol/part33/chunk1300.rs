//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1300/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1300(t28667: f64, t82736: f64, t23665: f64, t28626: f64, t1539: f64, t7582: f64, t82655: f64, t28622: f64, t225: f64, t28557: f64, t28565: f64, t6743: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99966 = t82736 * t28667;
    let t99977 = t23665 * t28626;
    let t100008 = t82655 * t1539 * t7582;
    let t100019 = t23665 * t28622;
    let t100126 = t28557 * t225;
    let t100137 = t28565 * t225;
    let t100148 = t28565 * t6743;
    (t99966, t99977, t100008, t100019, t100126, t100137, t100148)
}
