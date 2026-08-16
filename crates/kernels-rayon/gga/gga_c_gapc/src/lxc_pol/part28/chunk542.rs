//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 542/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk542(t3103: f64, t581: f64, t1720: f64, t458: f64, t178: f64, t568: f64, t116: f64, t19: f64, t147: f64, t5: f64, t3071: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3104 = t581 * t3103;
    let t3105 = t1720 * t458;
    let t3106 = t3104 * t3105;
    let t3108 = t178 * t3103;
    let t3109 = t1720 * t568;
    let t3110 = t3108 * t3109;
    let t3112 = t116 * t19;
    let t3113 = t147 * t5;
    let t3114 = t3113 * t3071;
    (t3104, t3105, t3106, t3108, t3109, t3110, t3112, t3113, t3114)
}
