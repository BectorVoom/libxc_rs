//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1295/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1295(t13796: f64, t3989: f64, t56296: f64, t875: f64, t1113: f64, t13776: f64, t3747: f64, t3975: f64, t810: f64, t46392: f64, t13781: f64, t3222: f64, t3886: f64, t3972: f64, param_a_c: f64) -> (f64, f64, f64, f64) {
    let t56701 = t3989 * t13796 * t56296 * t875;
    let t56708 = t13776 * t3975 * t1113 * t3747 * t810;
    let t56717 = t13776 * t3975 * t46392;
    let t56722 = t3972 * t13781 * t3886 * param_a_c * t3222;
    (t56701, t56708, t56717, t56722)
}
