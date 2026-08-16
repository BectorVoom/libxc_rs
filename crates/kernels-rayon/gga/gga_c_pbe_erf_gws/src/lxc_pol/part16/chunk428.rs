//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 428/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk428(t1597: f64, t242: f64, t528: f64, t700: f64, t1354: f64, t41: f64, t536: f64, t1383: f64, t148: f64, t1472: f64, t168: f64, t270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1598 = t1597 * t242;
    let t1601 = 0.16752564107100880375e0_f64 * t528 * t700;
    let t1602 = t41 * t1354;
    let t1605 = t536 * t700;
    let t1608 = 0.83762820535504401876e-1_f64 * t148 * t1383;
    let t1611 = 0.53059442957798955448e-1_f64 * t168 * t1472 * t270;
    (t1598, t1601, t1602, t1605, t1608, t1611)
}
