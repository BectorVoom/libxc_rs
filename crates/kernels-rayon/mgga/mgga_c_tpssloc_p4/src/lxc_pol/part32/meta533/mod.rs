//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1870;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta533(t27453: f64, t27454: f64, t1751: f64, t477: f64, t1090: f64, t7362: f64, t1653: f64, t24858: f64, t2144: f64, t5011: f64, t1246: f64, t4733: f64, t7363: f64, t1215: f64, t8054: f64, t1244: f64, t24760: f64, t24773: f64, t27406: f64, t27451: f64, t5064: f64, t7283: f64, t7365: f64, t7387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27455, t27460, t27461, t27462, t27465, t27466, t27471, t27473) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1870(t27453, t27454, t1751, t477, t1090, t7362, t1653, t24858, t2144, t5011, t1246, t4733, t7363);
        let (t27474, t27478, t27480) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1871(t27473, t7362, t1215, t8054, t1246, t1244, t24760, t24773, t27406, t27451, t27455, t27462, t27466, t27471, t5064, t7283, t7365, t7387);
    (t27455, t27460, t27461, t27462, t27465, t27466, t27471, t27473, t27474, t27478, t27480)
}
