//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1206/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1206(t3906: f64, t898: f64, t938: f64, t3886: f64, t6781: f64, t12232: f64, t810: f64, t3721: f64, t8734: f64, t12098: f64, t2376: f64, t2494: f64, t2501: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35889 = t3906 * t898;
    let t35890 = t35889 * t938;
    let t35910 = t6781 * t3886;
    let t36000 = t12232 * t810;
    let t36007 = t8734 * t3721;
    let t36046 = t2376 * t12098;
    let t36089 = t2501 * t2494;
    (t35889, t35890, t35910, t36000, t36007, t36046, t36089)
}
