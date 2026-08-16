//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2700/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2700<F: Float>(t571: F, t6330: F, t1297: F, t193: F, t40224: F, t40230: F, t54470: F, t54472: F, t54473: F, t54475: F, t54478: F, t74355: F, t74502: F, t74503: F, t74504: F) -> (F, F) {
    let t75256 = t6330 * t571;
    let t75267 = F::cast_from(3.0_f64) * t1297 * t193 * t74355 + t40224 - t40230 - t54470 - t54472 + t54473 - t54475 - t54478 - t74502 - t74503 + t74504;
    (t75256, t75267)
}
