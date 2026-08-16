//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2838/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2838(t11262: f64, t3161: f64, t3163: f64, t3147: f64, t3229: f64, t3141: f64, t3144: f64, t1036: f64, t11671: f64, t3278: f64, t2434: f64, t246: f64) -> (f64, f64, f64, f64, f64) {
    let t42932 = t3161 * t11262 * t3163;
    let t42937 = t3229 * t3147;
    let t42939 = t3141 * t3144 * t42937;
    let t42943 = t3141 * t1036 * t42937;
    let t42967 = t3278 * t11671;
    let t42994 = t246 * t2434;
    (t42932, t42939, t42943, t42967, t42994)
}
