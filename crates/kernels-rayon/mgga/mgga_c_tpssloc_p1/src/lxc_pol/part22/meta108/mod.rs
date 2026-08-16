//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta108 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk733;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk734;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk735;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk736;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk737;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk738;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta108(t3030: f64, t349: f64, t1011: f64, t68: f64, t371: f64, t335: f64, t368: f64, t1015: f64, t1030: f64, t372: f64, t364: f64, t354: f64, t1043: f64, t121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3031, t3032) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk733(t3030, t349, t1011, t68);
        let (t3033, t3034) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk734(t3031, t3032, t371);
        let t3036 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk735(t3034, t335);
        let (t3037, t3038) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk736(t3036, t368, t1015);
        let t3039 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk737(t3033, t3038);
        let (t3047, t3048) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk738(t1030, t372, t364, t354);
        let t3051 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk739(t1043, t121);
    (t3031, t3032, t3033, t3034, t3036, t3037, t3038, t3039, t3047, t3048, t3051)
}
