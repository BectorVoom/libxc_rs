//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 303/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk303(t1306: f64, t425: f64, t1265: f64, t169: f64, t172: f64, t452: f64, t118: f64, t422: f64) -> (f64, f64, f64) {
    let t1307 = t425 * t1306;
    let t1311 = t1265 * t169 * t172;
    let t1312 = t452 * t1311;
    let t1320 = 1.0_f64 / t422 / t118;
    (t1307, t1312, t1320)
}
