//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 656/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk656(t2610: f64, t5397: f64, t1865: f64, t296: f64, t1880: f64, t299: f64, t1710: f64, t1942: f64, t279: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t5398 = t2610 * t5397;
    let t5501 = t296 * t1865;
    let t5508 = t299 * t1880;
    let t5514 = t296 * t1710;
    let t5524 = t481 * t1942 * t279;
    (t5398, t5501, t5508, t5514, t5524)
}
