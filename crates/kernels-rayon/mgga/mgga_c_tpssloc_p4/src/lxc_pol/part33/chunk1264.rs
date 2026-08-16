//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1264/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1264(t1409: f64, t1862: f64, t605: f64, t111: f64, t27992: f64, t5464: f64, t81442: f64, t22470: f64, t5488: f64, t22674: f64, t28191: f64, t80681: f64) -> (f64, f64, f64, f64, f64) {
    let t96551 = t605 * t1409 * t1862;
    let t96686 = t27992 * t111;
    let t96713 = t81442 * t5464;
    let t96721 = t22470 * t5488;
    let t96848 = t80681 * t22674 * t28191;
    (t96551, t96686, t96713, t96721, t96848)
}
