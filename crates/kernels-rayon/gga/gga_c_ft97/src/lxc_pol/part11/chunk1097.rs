//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1097/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1097(t10036: f64, t1882: f64, t9845: f64, t10086: f64, t8392: f64, t2492: f64, t2542: f64, t10018: f64, t10052: f64, t10053: f64, t10121: f64, t13857: f64, t1901: f64, t193: f64, t1934: f64, t241: f64, t2469: f64, t2569: f64, t2574: f64, t258: f64, t2602: f64, t2606: f64, t265: f64, t42252: f64, t446: f64, t713: f64, t729: f64, t762: f64, t766: f64, t89: f64, t9692: f64) -> f64 {
    let t42961 = t1882 * t10036;
    let t42978 = t1882 * t9845;
    let t42994 = t8392 * t10086;
    let t42996 = t2492 * t2542;
    let t43005 = 8.0_f64 / 3.0_f64 * t42961 + 8.0_f64 * t446 * t729 * t10052 * t10053 * t713 + 8.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t9692 * t713 + 4.0_f64 / 3.0_f64 * t446 * t729 * t762 * t9692 * t766 - 8.0_f64 / 3.0_f64 * t42978 + 4.0_f64 * t446 * t729 * t2469 * t10018 + 4.0_f64 / 3.0_f64 * t446 * t729 * t762 * t10121 * t713 + t89 * t193 * t241 * t42252 * t258 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t42994 + 4.0_f64 / 3.0_f64 * t1901 * t42996 * t2602 - 4.0_f64 / 3.0_f64 * t1901 * t2606 * t13857 * t1934 * t2569;
    t43005
}
