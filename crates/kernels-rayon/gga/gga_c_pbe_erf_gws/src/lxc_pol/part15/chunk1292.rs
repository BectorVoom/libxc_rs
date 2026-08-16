//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1292/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1292(t14567: f64, t2118: f64, t27691: f64, t3074: f64, t3123: f64, t6674: f64, t2134: f64, t9127: f64, t14031: f64, t9434: f64, t9604: f64, t3116: f64, t51237: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53994 = t3074 * t2118 * t27691 * t14567;
    let t53996 = t3123 * t6674;
    let t53998 = t2134 * t9127;
    let t54000 = t14031 * t9434;
    let t54002 = t14031 * t9604;
    let t54004 = t3116 * t51237;
    (t53994, t53996, t53998, t54000, t54002, t54004)
}
