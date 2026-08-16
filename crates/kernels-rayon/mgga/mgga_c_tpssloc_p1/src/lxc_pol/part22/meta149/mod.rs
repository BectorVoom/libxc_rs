//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk946;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk947;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk948;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta149(t4496: f64, t4497: f64, t959: f64, t1592: f64, t2970: f64, t973: f64, t2978: f64, t60: f64, t344: f64, t4338: f64, t1409: f64, t2989: f64, t2988: f64, t2987: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4498, t4500, t4506, t4507, t4509) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk946(t4496, t4497, t959, t1592, t2970, t973, t2978, t60);
        let t4510 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk947(t344, t4509);
        let (t4511, t4514) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk948(t4338, t4510, t1409, t2989);
        let (t4515, t4518) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk949(t2988, t4514, t2987, t344);
    (t4498, t4500, t4506, t4507, t4509, t4510, t4511, t4514, t4515, t4518)
}
