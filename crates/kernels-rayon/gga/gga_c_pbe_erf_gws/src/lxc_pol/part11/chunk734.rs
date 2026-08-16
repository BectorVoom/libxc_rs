//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 734/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk734(t369: f64, t3772: f64, t3912: f64, t6216: f64, t11459: f64, t343: f64, t337: f64, t2121: f64, t2132: f64, t3747: f64, t1114: f64, t11478: f64, t2157: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11706 = t3772 * t369;
    let t11773 = t3912 * t6216;
    let t11776 = t11459 * t343;
    let t11777 = t337 * t11776;
    let t11778 = t2121 * t11777;
    let t11781 = t3747 * t2132;
    let t11782 = t1114 * t11781;
    let t11785 = t11478 * t2157;
    (t11706, t11773, t11777, t11778, t11781, t11782, t11785)
}
