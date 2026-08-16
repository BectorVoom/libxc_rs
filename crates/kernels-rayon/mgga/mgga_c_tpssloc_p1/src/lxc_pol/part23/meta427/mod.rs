//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1258;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1259;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1260;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1261;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1262;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta427(t22408: f64, t3640: f64, t20217: f64, t3242: f64, t21766: f64, t690: f64, t21773: f64, t21759: f64, t21770: f64, t21777: f64, t21763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71101, t71137, t71142) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1258(t22408, t3640, t20217, t3242, t21766, t690);
        let t71144 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1259(t21773, t690);
        let t71146 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1260(t21759, t690);
        let t71152 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1261(t21770, t690);
        let t71154 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1262(t21777, t690);
        let t71156 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1263(t21763, t690);
    (t71101, t71137, t71142, t71144, t71146, t71152, t71154, t71156)
}
