//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2706/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2706<F: Float>(t1307: F, t193: F, t39518: F, t39521: F, t39529: F, t39539: F, t54420: F, t54421: F, t54422: F, t54423: F, t54424: F, t54425: F, t54427: F) -> (F, F) {
    let t55224 = t193 * t1307;
    let t55228 = t54420 + t54421 - t54422 + t39518 - t39521 - t54423 - t39529 - t54424 - t54425 + t39539 - t54427;
    (t55224, t55228)
}
