//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1480;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta308(t14720: f64, t4775: f64, t699: f64, t11265: f64, t1661: f64, t11243: f64, t3270: f64, t4756: f64, t3287: f64, t4772: f64, t1657: f64, t3263: f64, t1098: f64, t4737: f64, t3312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14768, t14781, t14782, t14801, t14804, t14808, t14813, t14818, t14838) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1480(t14720, t4775, t699, t11265, t1661, t11243, t3270, t4756, t3287, t4772, t1657, t3263);
        let (t14845, t14850) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1481(t1098, t4737, t1657, t3312);
    (t14768, t14781, t14782, t14801, t14804, t14808, t14813, t14818, t14838, t14845, t14850)
}
