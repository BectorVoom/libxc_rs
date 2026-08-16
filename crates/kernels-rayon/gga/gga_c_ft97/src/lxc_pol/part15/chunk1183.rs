//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1183/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1183(t1701: f64, t290: f64, t6: f64, t2035: f64, t39: f64, t5230: f64, t1110: f64, t51: f64, t5284: f64, t1111: f64, t1209: f64, t14742: f64, t22003: f64, t22065: f64, t22081: f64, t22082: f64, t281: f64, t287: f64, t291: f64, t5003: f64, t5009: f64, t5265: f64, t5267: f64, t70463: f64, t70653: f64, t70779: f64, t83356: f64, t88439: f64, t88442: f64, t88909: f64, t90049: f64, t90159: f64) -> f64 {
    let t90280 = t290 * t6 * t1701;
    let t90288 = t5230 * t39 * t2035;
    let t90293 = t5284 * t6 * t51 * t1110;
    let t90300 = 0.45910941751869106328e2_f64 * t22082 * t5003 + 0.22341601828860387373e3_f64 * t5265 * t5009 * t88909 * t291 + 0.14498192132169191472e2_f64 * t22081 * t1209 * t1111 - 0.14498192132169191472e2_f64 * t22065 * t1111 + 0.19686723316703981795e0_f64 * t281 * t88439 * t88442 * t287 * t90280 - 0.14498192132169191472e2_f64 * t14742 * t90159 + 0.70065858367097548785e2_f64 * t70779 * t90049 + 0.87582322958871935983e1_f64 * t90288 * t5267 - 0.28996384264338382944e2_f64 * t70653 * t90293 + 0.28996384264338382944e2_f64 * t70463 * t90293 + 0.14498192132169191472e2_f64 * t83356 * t22003;
    t90300
}
