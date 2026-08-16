//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta133 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk746;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk747;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk748;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk749;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk750;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk751;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta133(t1208: f64, t476: f64, t478: f64, t3036: f64, t483: f64, t3500: f64, t475: f64, t1210: f64, t121: f64, t1229: f64, t1090: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3502 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk746(t1208, t476);
        let t3503 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk747(t3502, t478);
        let (t3504, t3505, t3506) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk748(t3036, t483, t3503, t3500);
        let t3508 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk749(t475);
        let (t3514, t3515) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk750(t1210, t3504, t3500);
        let t3521 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk751(t121, t1229);
        let t3523 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk752(t1090, t248, t3521);
    (t3502, t3503, t3504, t3505, t3506, t3508, t3514, t3515, t3521, t3523)
}
