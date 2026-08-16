//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 561/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk561(t331: f64, t991: f64, t551: f64, t553: f64, t1052: f64, t163: f64, t169: f64, t299: f64, t1049: f64, t230: f64, t225: f64, t2522: f64) -> (f64, f64, f64, f64, f64) {
    let t2948 = t331 * t991;
    let t2950 = t2948 * t551 * t553;
    let t2957 = t169 * t299 * t1052 * t163;
    let t2960 = t1049 * t230;
    let t2962 = t2522 * t225;
    (t2948, t2950, t2957, t2960, t2962)
}
