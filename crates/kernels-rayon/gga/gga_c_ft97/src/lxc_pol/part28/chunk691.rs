//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 691/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk691(t23900: f64, t920: f64, t1969: f64, t446: f64, t358: f64, t6615: f64, t363: f64, t558: f64, t6630: f64, t9432: f64, t18: f64, t5916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27030 = t23900 * t920;
    let t27031 = t1969 * t27030;
    let t27032 = t446 * t27031;
    let t27034 = t6615 * t358;
    let t27035 = t27034 * t363;
    let t27036 = t1969 * t27035;
    let t27037 = t446 * t27036;
    let t27040 = t9432 * t6630 * t558;
    let t27041 = t446 * t27040;
    let t27043 = t5916 * t18;
    (t27030, t27032, t27035, t27037, t27041, t27043)
}
