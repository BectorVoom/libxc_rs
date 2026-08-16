//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta152 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk986;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk987;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk988;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk989;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk990;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk991;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta152(t3590: f64, t466: f64, t1236: f64, t225: f64, t1239: f64, t496: f64, t68: f64, t1251: f64, t1243: f64, t3534: f64, t3032: f64, t3502: f64, t3499: f64, t3507: f64, t491: f64, t1932: f64, t3508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3591, t3593) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk986(t3590, t466, t1236, t225);
        let t3598 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk987(t1239, t496, t68);
        let (t3599, t3600) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk988(t1251, t3598);
        let t3604 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk989(t1243, t3534);
        let t3609 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk990(t3032, t3502);
        let t3610 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk991(t3499, t3609);
        let (t3611, t3612) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk992(t3507, t491, t1932, t3508);
    (t3591, t3593, t3598, t3599, t3600, t3604, t3609, t3610, t3611, t3612)
}
