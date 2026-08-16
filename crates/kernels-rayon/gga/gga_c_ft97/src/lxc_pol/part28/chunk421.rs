//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 421/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk421(t1317: f64, t28: f64, t6508: f64, t5691: f64, t920: f64, t1564: f64, t446: f64, t5507: f64, t942: f64, t89: f64, t370: f64, t6454: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6510 = t1317 * t28 * t6508;
    let t6512 = t5691 * t920;
    let t6513 = t1564 * t6512;
    let t6514 = t446 * t6513;
    let t6516 = t5507 * t942;
    let t6517 = t28 * t6516;
    let t6518 = t89 * t6517;
    let t6520 = t370 * t6454;
    (t6510, t6512, t6513, t6514, t6516, t6517, t6518, t6520)
}
