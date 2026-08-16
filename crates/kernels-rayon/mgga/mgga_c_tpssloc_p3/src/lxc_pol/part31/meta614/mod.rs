//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta614(t27975: f64, t645: f64, t72: f64, t1864: f64, t5445: f64, t2240: f64, t5399: f64, t3953: f64, t3961: f64, t3967: f64, t1437: f64, t4017: f64, t1433: f64, t4021: f64, t641: f64, t19445: f64, t79: f64, t19299: f64, t608: f64, t3966: f64, t2235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96466, t96469, t96473, t96479, t96482, t96502) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1860(t27975, t645, t72, t1864, t5445, t2240, t5399, t3953, t3961, t3967, t1437, t4017);
        let (t96506, t96517, t96521, t96535, t96553, t96556) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1861(t1433, t4021, t72, t5445, t641, t19445, t79, t19299, t608, t3966, t2235, t5399);
    (t96466, t96469, t96473, t96479, t96482, t96502, t96506, t96517, t96521, t96535, t96553, t96556)
}
