//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 944/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk944(t2522: f64, t331: f64, t551: f64, t553: f64, t1371: f64, t2948: f64, t1378: f64, t1971: f64, t8361: f64, t163: f64, t169: f64, t2979: f64, t299: f64) -> (f64, f64, f64, f64) {
    let t8382 = t331 * t2522;
    let t8385 = 0.39507780657818961764e-2_f64 * t8382 * t551 * t553;
    let t8387 = t2948 * t1371 * t553;
    let t8390 = t8361 * t1378 * t1971;
    let t8395 = 0.17961351015381913641e-1_f64 * t169 * t299 * t2979 * t163;
    (t8385, t8387, t8390, t8395)
}
