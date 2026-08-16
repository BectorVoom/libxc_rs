//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 808/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk808(t25359: f64, t2615: f64, t9438: f64, t2344: f64, t550: f64, t1358: f64, t161: f64, t37975: f64, t11280: f64, t20883: f64, t6525: f64, t42539: f64) -> (f64, f64, f64, f64, f64) {
    let t44133 = t2615 * t9438 * t25359;
    let t44255 = t550 * t2344;
    let t44258 = 0.37940008847568199464e-1_f64 * t1358 * t37975 * t161 * t44255;
    let t44261 = t6525 * t11280 * t20883;
    let t44262 = 0.35568758294595186999e-2_f64 * t44261;
    let t44263 = 0.47425011059460249332e-2_f64 * t42539;
    (t44133, t44255, t44258, t44262, t44263)
}
