//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 991/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk991(t1114: f64, t3747: f64, t4422: f64, t833: f64, t3703: f64, t898: f64, t3717: f64, t20091: f64, t3744: f64, t3906: f64, t3898: f64, t4442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35481 = t1114 * t3747 * t4422 * t833;
    let t35541 = t898 * t3703;
    let t35553 = t898 * t3717;
    let t35638 = t20091 * t3744;
    let t35889 = t3906 * t898;
    let t35929 = t4442 * t3898;
    (t35481, t35541, t35553, t35638, t35889, t35929)
}
