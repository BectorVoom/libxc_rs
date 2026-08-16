//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk853;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk854;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk855;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk856;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk857;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk858;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk859;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk860;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta161(t3502: f64, t478: f64, t3036: f64, t483: f64, t3500: f64, t1215: f64, t475: f64, t1214: f64, t248: f64, t1210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3503 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk853(t3502, t478);
        let (t3504, t3505, t3506) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk854(t3036, t483, t3503, t3500);
        let t3507 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk855(t1215);
        let t3508 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk856(t475);
        let t3509 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk857(t3507, t3508);
        let t3511 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk858(t1214, t248, t3509);
        let (t3514, t3515) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk859(t1210, t3504, t3500);
        let t3516 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk860(t3507, t475);
        let t3518 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk861(t1214, t248, t3516);
    (t3503, t3504, t3505, t3506, t3507, t3508, t3509, t3511, t3514, t3515, t3516, t3518)
}
