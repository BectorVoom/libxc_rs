//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1246;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1247;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1248;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1249;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta346(t2784: f64, t2841: f64, t2845: f64, t10697: f64, t2787: f64, t10696: f64, t2842: f64, t2844: f64, t912: f64, t10702: f64, t10704: f64, t2793: f64, t2836: f64, t2775: f64, t39103: f64, t123: f64, t882: f64, t10249: f64, t9258: f64, t10277: f64, t2244: f64, t2250: f64, t2768: f64, t22715: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41625, t41627, t41635, t41639) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1246(t2784, t2841, t2845, t10697, t2787, t10696, t2842, t2844, t912, t10702, t10704, t2793, t2836);
        let (t41640, t41642) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1247(t2775, t39103, t123, t882);
        let (t41644, t41646) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1248(t10249, t9258, t123, t882);
        let (t41649, t41651) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1249(t10277, t2244, t2250, t123, t2768);
        let t41654 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1250(t22715, t268, t271);
    (t41625, t41627, t41635, t41639, t41640, t41642, t41644, t41646, t41649, t41651, t41654)
}
