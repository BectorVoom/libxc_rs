//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 761/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk761(t29853: f64, t6508: f64, t122: f64, t2310: f64, t481: f64, t158: f64, t9127: f64, t3085: f64, t447: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29854 = t6508 * t29853;
    let t29874 = t481 * t2310 * t122;
    let t29882 = t158 * t9127;
    let t29969 = t3085 * t447;
    let t29970 = t6508 * t29969;
    let t29975 = t3085 * t475;
    (t29854, t29874, t29882, t29969, t29970, t29975)
}
