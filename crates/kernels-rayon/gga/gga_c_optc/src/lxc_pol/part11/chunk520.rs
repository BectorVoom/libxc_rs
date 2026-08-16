//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 520/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk520(t1382: f64, t288: f64, t3829: f64, t953: f64, t1378: f64, t2367: f64, t930: f64, t116: f64, t195: f64) -> (f64, f64, f64, f64) {
    let t3885 = t288 * t1382;
    let t3892 = t953 * t3829;
    let t3896 = t2367 * t1378;
    let t3897 = t930 * t3896;
    let t3902 = t116 * t195;
    (t3885, t3892, t3897, t3902)
}
