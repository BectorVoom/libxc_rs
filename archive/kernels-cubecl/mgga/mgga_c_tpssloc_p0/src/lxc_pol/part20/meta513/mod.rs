//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2032;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2033;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2034;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2035;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2036;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2037;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta513<F: Float>(t2409: F, t2413: F, t125: F, t39253: F, t2414: F, t9479: F, t11985: F, t526: F, t11998: F, t528: F, t2405: F, t2419: F, t690: F, t703: F, t2410: F, t701: F, t268: F, t682: F, t781: F, t204: F, t2421: F, t12083: F, t172: F, t763: F, t2411: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t39408 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2032::<F>(t2409, t2413, t125, t39253);
        let t39411 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2033::<F>(t2414, t39253, t9479);
        let (t39419, t39436, t39463) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2034::<F>(t11985, t526, t11998, t528, t2405, t2419, t690, t703);
        let t39468 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2035::<F>(t2405, t2410, t2414, t690, t701);
        let t39472 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2036::<F>(t268, t682, t703, t781);
        let t39476 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2037::<F>(t204, t2419, t2421, t268);
        let (t39478, t39483) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2038::<F>(t12083, t172, t763, t2405, t2411, t2421);
    (t39408, t39411, t39419, t39436, t39463, t39468, t39472, t39476, t39478, t39483)
}
