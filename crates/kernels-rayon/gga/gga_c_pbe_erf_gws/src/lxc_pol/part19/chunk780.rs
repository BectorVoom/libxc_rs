//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 780/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk780(t1368: f64, t285: f64, t762: f64, t147: f64, t366: f64, t169: f64, t242: f64, t535: f64, t784: f64, t1339: f64, t700: f64, t1343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5694 = 0.87170224553660758101e-3_f64 * t762 * t1368 * t285;
    let t5697 = t366 * t147;
    let t5700 = 0.5188034422540342311e0_f64 * t169 * t5697 * t242;
    let t5701 = t784 * t535;
    let t5703 = t169 * t5701 * t242;
    let t5707 = 0.42447554366239164361e0_f64 * t169 * t1339 * t700;
    let t5713 = t169 * t1343 * t700;
    (t5694, t5697, t5700, t5701, t5703, t5707, t5713)
}
