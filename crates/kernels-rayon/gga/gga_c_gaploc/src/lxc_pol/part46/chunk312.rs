//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 312/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk312(t169: f64, t2530: f64, t299: f64, t706: f64, t296: f64, t935: f64) -> (f64, f64, f64) {
    let t2531 = t2530 * t169;
    let t2532 = t2531 * t299;
    let t2533 = t706 * t2532;
    let t2536 = t296 * t935;
    (t2532, t2533, t2536)
}
