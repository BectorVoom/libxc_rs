//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta186 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk905;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk906;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk907;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk908;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk909;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta186(t1734: f64, t486: f64, t1215: f64, t3508: f64, t4582: f64, t1216: f64, t3242: f64, t3584: f64, t3961: f64, t1653: f64, t248: f64, t3521: f64, t1227: f64, t1735: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4977, t4978) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk905(t1734, t486, t1215, t3508);
        let (t4979, t4980) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk906(t4977, t4978, t4582);
        let (t4983, t4984) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk907(t1216, t4977, t4582);
        let (t4987, t4988, t4989) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk908(t3242, t3584, t3961, t4582);
        let t4993 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk909(t1653, t248, t3521);
        let (t4994, t4997) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk910(t1227, t4993, t1735, t248, t3570);
    (t4977, t4978, t4979, t4980, t4983, t4984, t4987, t4988, t4989, t4993, t4994, t4997)
}
