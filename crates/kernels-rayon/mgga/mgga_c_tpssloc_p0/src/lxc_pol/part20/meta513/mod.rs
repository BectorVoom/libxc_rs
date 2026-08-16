//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2032;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2033;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2034;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2035;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2036;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2037;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta513(t2409: f64, t2413: f64, t125: f64, t39253: f64, t2414: f64, t9479: f64, t11985: f64, t526: f64, t11998: f64, t528: f64, t2405: f64, t2419: f64, t690: f64, t703: f64, t2410: f64, t701: f64, t268: f64, t682: f64, t781: f64, t204: f64, t2421: f64, t12083: f64, t172: f64, t763: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39408 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2032(t2409, t2413, t125, t39253);
        let t39411 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2033(t2414, t39253, t9479);
        let (t39419, t39436, t39463) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2034(t11985, t526, t11998, t528, t2405, t2419, t690, t703);
        let t39468 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2035(t2405, t2410, t2414, t690, t701);
        let t39472 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2036(t268, t682, t703, t781);
        let t39476 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2037(t204, t2419, t2421, t268);
        let (t39478, t39483) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2038(t12083, t172, t763, t2405, t2411, t2421);
    (t39408, t39411, t39419, t39436, t39463, t39468, t39472, t39476, t39478, t39483)
}
