//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1316/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1316(t13984: f64, t56104: f64, t11438: f64, t13917: f64, t2249: f64, t53446: f64, t3765: f64, t51465: f64, t11414: f64, t2134: f64, t12021: f64, t14031: f64) -> (f64, f64, f64, f64, f64) {
    let t56849 = t56104 * t13984;
    let t56853 = t13917 * t2249 * t53446 * t11438;
    let t56855 = t51465 * t3765;
    let t56857 = t2134 * t11414;
    let t56859 = t14031 * t12021;
    (t56849, t56853, t56855, t56857, t56859)
}
