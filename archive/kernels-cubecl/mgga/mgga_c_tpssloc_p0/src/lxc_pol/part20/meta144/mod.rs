//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk925;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk926;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk927;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk928;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta144<F: Float>(t3242: F, t461: F, t2244: F, t3440: F, t337: F, t51: F, t1887: F, t1176: F, t60: F, t1184: F, t1089: F, t460: F, t607: F) -> (F, F, F, F, F, F, F, F) {
        let (t3441, t3442, t3443, t3447) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk925::<F>(t3242, t461, t2244, t3440, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk926::<F>(t1176, t60);
        let t3449 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk927::<F>(t1184, t3448);
        let t3450 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk928::<F>(t1089, t460);
        let t3451 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk929::<F>(t3450, t607);
    (t3441, t3442, t3443, t3447, t3448, t3449, t3450, t3451)
}
