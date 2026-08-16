//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1575;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta437(t23122: f64, t23124: f64, t2617: f64, t6620: f64, t6619: f64, t835: f64, t812: f64, t849: f64, t1891: f64, t9223: f64, t213: f64, t1895: f64, t1887: f64, t206: f64, t22715: f64, t242: f64, t6612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23125, t23127, t23132, t23133) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1575(t23122, t23124, t2617, t6620, t6619, t835, t812);
        let (t23135, t23138, t23140, t23143, t23145, t23146) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1576(t23133, t849, t1891, t9223, t213, t1895, t1887, t206, t22715, t242, t6612, t812);
    (t23125, t23127, t23132, t23133, t23135, t23138, t23140, t23143, t23145, t23146)
}
