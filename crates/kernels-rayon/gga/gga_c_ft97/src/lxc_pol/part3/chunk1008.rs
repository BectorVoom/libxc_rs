//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1008/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1008(t4973: f64, t835: f64, t882: f64, t18123: f64, t319: f64, t2857: f64, t4965: f64, t1091: f64, t4181: f64, t15312: f64, t1248: f64, t505: f64) -> (f64, f64, f64, f64, f64) {
    let t19606 = t835 * t882 * t4973;
    let t19610 = t835 * t319 * t18123;
    let t19614 = t2857 * t882 * t4965;
    let t19617 = t1091 * t4181;
    let t19618 = t15312 * t19617;
    let t19621 = t1248 * t505;
    (t19606, t19610, t19614, t19618, t19621)
}
