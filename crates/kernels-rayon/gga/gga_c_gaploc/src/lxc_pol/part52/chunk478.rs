//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 478/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk478(t158: f64, t2754: f64, t2796: f64, t501: f64, t1381: f64, t997: f64, t2876: f64, t540: f64, t1: f64, t106: f64, t192: f64, t1564: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8025 = t158 * t2754;
    let t8042 = t2796 * t501;
    let t8045 = t997 * t1381;
    let t8063 = t2876 * t540;
    let t8070 = t2754 * t1;
    let t8071 = t8070 * t106;
    let t8072 = t8071 * t192;
    let t8097 = t1564 * t2754;
    (t8025, t8042, t8045, t8063, t8072, t8097)
}
