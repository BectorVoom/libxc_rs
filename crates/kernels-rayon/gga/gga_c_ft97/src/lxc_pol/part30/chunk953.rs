//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 953/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk953(t258: f64, t33452: f64, t1882: f64, t33738: f64, t33489: f64, t761: f64, t7548: f64, t8232: f64, t33683: f64, t7484: f64, t2492: f64, t7536: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t141902 = t258 * t33452;
    let t141914 = t1882 * t33738;
    let t141916 = t761 * t33489;
    let t141942 = 8.0_f64 / 27.0_f64 * t8232 * t7548;
    let t141947 = t1882 * t33683;
    let t141989 = t761 * t7484;
    let t141997 = t2492 * t7536;
    (t141902, t141914, t141916, t141942, t141947, t141989, t141997)
}
