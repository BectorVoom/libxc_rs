//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 983/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk983(t4395: f64, t8652: f64, t3074: f64, t2379: f64, t3083: f64, t2081: f64, t326: f64, t6469: f64, t3075: f64, t6472: f64, t1161: f64, t2416: f64) -> (f64, f64, f64, f64, f64) {
    let t8775 = t4395 * t8652;
    let t8776 = t3074 * t8775;
    let t8780 = 7.0_f64 / 144.0_f64 * t3083 * t2379;
    let t8782 = t326 * t6469 * t2081;
    let t8784 = t8782 * t6472 * t3075;
    let t8787 = t2416 * t1161;
    (t8776, t8780, t8782, t8784, t8787)
}
