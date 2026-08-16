//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 687/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk687(t2360: f64, t317: f64, t3886: f64, t28938: f64, t1477: f64, t2404: f64, t2347: f64, t684: f64, t6970: f64, t25412: f64, t4309: f64, t193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28939 = t317 * t2360;
    let t28940 = t28939 * t3886;
    let t28941 = t28938 * t28940;
    let t28944 = t2404 * t1477;
    let t28945 = t317 * t2347;
    let t28946 = t28945 * t3886;
    let t28947 = t28944 * t28946;
    let t28950 = t6970 * t684;
    let t28951 = t25412 * t28950;
    let t28954 = t1477 * t4309;
    let t28955 = t193 * t28954;
    (t28940, t28941, t28944, t28946, t28947, t28950, t28951, t28955)
}
