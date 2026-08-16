//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1916;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta645(t22986: f64, t23270: f64, t25191: f64, t4300: f64, t25192: f64, t86873: f64, t5544: f64, t857: f64, t865: f64, t1527: f64, t86849: f64, t4272: f64, t86969: f64, t1520: f64, t254: f64, t25038: f64, t25039: f64, t4119: f64, t1880: f64, t7488: f64, t87782: f64, t23237: f64, t28276: f64, t6552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98248, t98251, t98256, t98264, t98277) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1916(t22986, t23270, t25191, t4300, t25192, t86873, t5544, t857, t865, t1527, t86849, t4272, t86969);
        let (t98279, t98291, t98305, t98315) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1917(t1520, t254, t23270, t25038, t25039, t4119, t1880, t7488, t87782, t23237, t28276, t6552);
    (t98248, t98251, t98256, t98264, t98277, t98279, t98291, t98305, t98315)
}
