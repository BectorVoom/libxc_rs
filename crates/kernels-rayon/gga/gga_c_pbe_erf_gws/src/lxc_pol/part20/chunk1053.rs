//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1053/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1053(t11819: f64, t9343: f64, t2255: f64, t3111: f64, t3752: f64, t3037: f64, t816: f64, t3257: f64, t3258: f64, t3116: f64, t8874: f64, t337: f64, t3791: f64, t814: f64) -> (f64, f64, f64, f64, f64) {
    let t11820 = t9343 * t11819;
    let t11824 = t2255 * t3111 * t3752;
    let t11827 = t816 * t3037;
    let t11829 = t3257 * t3258 * t11827;
    let t11833 = t3116 * t8874 / 24.0_f64;
    let t11835 = t337 * t3791 * t814;
    (t11820, t11824, t11829, t11833, t11835)
}
