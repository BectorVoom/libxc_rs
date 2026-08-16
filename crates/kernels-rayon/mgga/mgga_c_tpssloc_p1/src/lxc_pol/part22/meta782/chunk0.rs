//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2672/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2672(t54462: f64, t39851: f64, t54467: f64, t57227: f64, t57229: f64, t57235: f64, t40224: f64, t40230: f64, t54459: f64, t54461: f64, t54465: f64, t54466: f64, t54470: f64, t54472: f64, t54473: f64, t54475: f64, t54478: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t74499 = 360.0_f64 * t54462;
    let t74500 = 12.0_f64 * t39851;
    let t74501 = 0.30762056574649219972e4_f64 * t54467;
    let t74502 = 12.0_f64 * t57227;
    let t74503 = 12.0_f64 * t57229;
    let t74504 = 0.32530743900905219526e-1_f64 * t57235;
    let t74505 = t54459 - t54461 - t74499 - t74500 - t54465 + t54466 - t74501 - t54470 - t54472 + t40224 + t54473 - t74502 - t74503 - t54475 - t40230 + t74504 - t54478;
    (t74499, t74500, t74501, t74502, t74503, t74504, t74505)
}
