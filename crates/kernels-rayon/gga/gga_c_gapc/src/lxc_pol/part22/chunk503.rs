//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 503/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk503(t116: f64, t1457: f64, t2920: f64, t134: f64, t190: f64, t1954: f64, t200: f64, t1475: f64, t996: f64, t493: f64, t568: f64, t1004: f64, t423: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2921 = t116 * t1457;
    let t2922 = t2920 * t2921;
    let t2923 = t190 * t134;
    let t2925 = t2923 * t200 * t1954;
    let t2926 = t2922 * t2925;
    let t2928 = t996 * t1475;
    let t2929 = t493 * t568;
    let t2930 = t2928 * t2929;
    let t2932 = t1004 * t423;
    (t2921, t2922, t2923, t2925, t2926, t2928, t2929, t2930, t2932)
}
