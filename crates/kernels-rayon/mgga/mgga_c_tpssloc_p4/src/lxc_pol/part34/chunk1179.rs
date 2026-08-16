//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1179/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1179(t1527: f64, t23270: f64, t25038: f64, t98224: f64, t1484: f64, t5664: f64, t25373: f64, t5397: f64, t1408: f64, t5544: f64, t5660: f64, t22960: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105698 = t25038 * t23270 * t98224 * t1527;
    let t105731 = t1484 * t5664;
    let t105732 = t25373 * t105731;
    let t105741 = t5397 * t1484;
    let t105745 = t1408 * t5544;
    let t105754 = t1484 * t5660;
    let t105755 = t22960 * t105754;
    (t105698, t105731, t105732, t105741, t105745, t105754, t105755)
}
