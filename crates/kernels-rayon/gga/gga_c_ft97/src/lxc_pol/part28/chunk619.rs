//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 619/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk619(t358: f64, t6454: f64, t363: f64, t1564: f64, t446: f64, t18: f64, t5691: f64, t3281: f64, t432: f64, t6469: f64, t8411: f64, t25933: f64, t7824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25955 = t6454 * t358;
    let t25956 = t25955 * t363;
    let t25957 = t1564 * t25956;
    let t25958 = t446 * t25957;
    let t25960 = t5691 * t18;
    let t25961 = t1564 * t25960;
    let t25962 = t3281 * t25961;
    let t25965 = t8411 * t6469 * t432;
    let t25966 = t446 * t25965;
    let t25969 = t7824 * t25933;
    (t25956, t25958, t25960, t25962, t25966, t25969)
}
