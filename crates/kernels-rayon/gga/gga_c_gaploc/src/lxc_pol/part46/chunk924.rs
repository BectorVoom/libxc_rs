//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 924/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk924(t13096: f64, t325: f64, t550: f64, t296: f64, t10687: f64, t2554: f64, t7064: f64, t13200: f64, t29439: f64, t3247: f64, t32692: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42920 = t325 * t13096;
    let t42921 = t550 * t42920;
    let t42925 = t296 * t13096;
    let t42931 = t7064 * t10687 * t2554;
    let t42933 = t29439 * t13200;
    let t42934 = 0.1922631557535556071e-2_f64 * t42933;
    let t42936 = t9647 * t32692 * t3247;
    (t42920, t42921, t42925, t42931, t42934, t42936)
}
