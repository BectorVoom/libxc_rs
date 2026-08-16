//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2831/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2831(t3105: f64, t3223: f64, t1041: f64, t11262: f64, t3135: f64, t12166: f64, t15905: f64, t994: f64, t11631: f64, t999: f64, t3046: f64, t3298: f64, t4891: f64) -> (f64, f64, f64, f64, f64) {
    let t42571 = t3223 * t3105;
    let t42580 = t1041 * t11262 * t3135;
    let t42621 = t994 * t12166 * t15905;
    let t42622 = t11631 * t999;
    let t42643 = t3046 * t3298 * t4891;
    (t42571, t42580, t42621, t42622, t42643)
}
