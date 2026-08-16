//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 772/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk772(t10600: f64, t1415: f64, t31590: f64, t493: f64, t26126: f64, t544: f64, t18535: f64, t19: f64, t584: f64, t60: f64, t18540: f64, t201: f64) -> (f64, f64, f64, f64, f64) {
    let t34264 = t1415 * t10600;
    let t34273 = t493 * t31590;
    let t34286 = t544 * t26126;
    let t34400 = t584 * t18535 * t19 * t60;
    let t34401 = t201 * t18540;
    (t34264, t34273, t34286, t34400, t34401)
}
