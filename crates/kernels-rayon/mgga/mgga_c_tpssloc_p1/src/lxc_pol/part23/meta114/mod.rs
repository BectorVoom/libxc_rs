//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk594;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta114(t1580: f64, t2904: f64, t1592: f64, t2970: f64, t973: f64, t2978: f64, t60: f64, t344: f64, t1409: f64, t2989: f64, t2987: f64, t135: f64, t1599: f64, t1597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4488, t4507, t4509, t4510, t4514) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk594(t1580, t2904, t1592, t2970, t973, t2978, t60, t344, t1409, t2989);
        let (t4518, t4529, t4531) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk595(t2987, t344, t135, t1599, t973, t1597);
    (t4488, t4507, t4509, t4510, t4514, t4518, t4529, t4531)
}
