//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2329/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2329(t100637: f64, t100818: f64, t113: f64, t20100: f64, t20136: f64, t510: f64, t6517: f64, t96654: f64, t97910: f64, t97914: f64, t97916: f64, t97919: f64, t97923: f64, t97925: f64, t97928: f64, t97930: f64, t97932: f64, t97935: f64, t97937: f64, t97941: f64, t97942: f64, t97947: f64, t97949: f64) -> f64 {
    let t100822 = -t97910 + t97914 - t97916 - t97919 + t97923 + t97925 - t97928 + t97930 - t97932 - t97935 - t97937 - 2.0_f64 * t6517 * t20100 + t97941 + t97942 - 4.0_f64 * t6517 * t20136 - t97947 - t97949 - t113 * (t100637 + t100818) - t96654 * t510;
    t100822
}
