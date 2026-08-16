//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1592;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1593;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta322(t11620: f64, t1246: f64, t1235: f64, t3507: f64, t3625: f64, t1155: f64, t3375: f64, t3396: f64, t1164: f64, t11128: f64, t11133: f64, t11179: f64, t11182: f64, t11184: f64, t11187: f64, t11405: f64, t11409: f64, t11426: f64, t11429: f64, t3395: f64, t3400: f64, t4883: f64, t11194: f64, t11272: f64, t11280: f64, t11288: f64, t11290: f64, t11296: f64, t11472: f64, t11475: f64, t11480: f64, t11482: f64, t11484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11621, t11624, t11625, t11629, t11631, t11632) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1592(t11620, t1246, t1235, t3507, t3625, t1155, t3375, t3396, t1164, t11128, t11133, t11179, t11182, t11184, t11187, t11405, t11409, t11426, t11429);
        let (t11634, t11636, t11637) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1593(t3395, t3400, t4883, t1164, t11194, t11272, t11280, t11288, t11290, t11296, t11472, t11475, t11480, t11482, t11484);
        let t11638 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1594(t11632, t11637);
    (t11621, t11624, t11625, t11629, t11631, t11634, t11636, t11638)
}
