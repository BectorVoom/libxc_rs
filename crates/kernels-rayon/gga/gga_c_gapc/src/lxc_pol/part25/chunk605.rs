//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 605/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk605(t1117: f64, t883: f64, t1125: f64, t972: f64, t3274: f64, t3276: f64, t3279: f64, t3282: f64, t3286: f64, t3290: f64, t3298: f64, t3301: f64, t3305: f64, t3308: f64, t3310: f64) -> (f64, f64, f64) {
    let t3565 = t1117 * t883;
    let t3568 = t1125 * t972;
    let t3582 = -0.3373480902777777778e-5_f64 * t3274 - 0.16908181191593721013e-4_f64 * t3276 + 0.14492726735651760868e-5_f64 * t3279 + 0.12357942809624928455e-3_f64 * t3282 + 0.28985453471303521736e-5_f64 * t3286 - 0.28985453471303521736e-5_f64 * t3290 + 0.14758978949652777779e-5_f64 * t3298 - 0.50680539737635041235e-4_f64 * t3301 - 0.14492726735651760868e-5_f64 * t3305 + 0.27801896084645508334e-2_f64 * t3308 + 0.27801896084645508334e-2_f64 * t3310;
    (t3565, t3568, t3582)
}
