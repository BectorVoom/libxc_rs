//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2147/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2147<F: Float>(t10908: F, t3109: F, t1036: F, t10446: F, t10997: F, t135: F, t973: F, t10480: F, t10483: F, t248: F, t3101: F, t10876: F, t10877: F) -> (F, F, F, F, F) {
    let t43254 = t3109 * t10908;
    let t43262 = t10446 * t1036;
    let t43273 = t973 * t135 * t10997;
    let t43277 = t10480 * t248 * t3101 * t10483;
    let t43281 = t10876 * t248 * t3101 * t10877;
    (t43254, t43262, t43273, t43277, t43281)
}
