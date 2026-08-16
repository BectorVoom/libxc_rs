//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 794/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk794(t13200: f64, t29439: f64, t3247: f64, t32692: f64, t9647: f64, t10697: f64, t9624: f64, t2558: f64, t33348: f64, t13182: f64, t1841: f64, t2563: f64, t3487: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42933 = t29439 * t13200;
    let t42936 = t9647 * t32692 * t3247;
    let t42939 = t9647 * t10697 * t9624;
    let t42942 = t9647 * t33348 * t2558;
    let t42953 = t1841 * t13182;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    (t42933, t42936, t42939, t42942, t42953, t42960)
}
