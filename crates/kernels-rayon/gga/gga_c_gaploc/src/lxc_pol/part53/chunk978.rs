//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 978/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk978(t13848: f64, t7416: f64, t12255: f64, t769: f64, t3470: f64, t313: f64, t39403: f64, t12223: f64, t2464: f64, t2465: f64, t825: f64, t39002: f64, t787: f64) -> (f64, f64, f64, f64, f64) {
    let t47494 = t7416 * t13848;
    let t47496 = t769 * t12255;
    let t47497 = t47496 * t3470;
    let t47500 = t313 * t39403;
    let t47501 = t47500 * t3470;
    let t47506 = t825 * t2464 * t2465 * t12223;
    let t47508 = t787 * t39002;
    (t47494, t47497, t47501, t47506, t47508)
}
