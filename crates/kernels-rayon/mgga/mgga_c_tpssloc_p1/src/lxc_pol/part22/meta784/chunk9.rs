//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2700/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2700(t571: f64, t6330: f64, t1297: f64, t193: f64, t40224: f64, t40230: f64, t54470: f64, t54472: f64, t54473: f64, t54475: f64, t54478: f64, t74355: f64, t74502: f64, t74503: f64, t74504: f64) -> (f64, f64) {
    let t75256 = t6330 * t571;
    let t75267 = 3.0_f64 * t1297 * t193 * t74355 + t40224 - t40230 - t54470 - t54472 + t54473 - t54475 - t54478 - t74502 - t74503 + t74504;
    (t75256, t75267)
}
