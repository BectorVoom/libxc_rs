//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1347/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1347(t14188: f64, t26958: f64, t353: f64, t4228: f64, t814: f64, t859: f64, t14888: f64, t19906: f64, t1206: f64, t3200: f64, t335: f64, t338: f64, t4111: f64, t4385: f64, t51807: f64, t51819: f64, t51827: f64, t51829: f64, t52241: f64, t52600: f64, t53910: f64, t53925: f64, t53930: f64, t53936: f64, t8629: f64, t8793: f64, t8939: f64, t9241: f64, t9283: f64) -> f64 {
    let t55695 = 7.0_f64 / 72.0_f64 * t26958 * t14188;
    let t55698 = t859 * t353 * t4228 * t814;
    let t55702 = 7.0_f64 / 72.0_f64 * t19906 * t14888;
    let t55703 = t9241 * t9283 * t1206 * t8939 / 4.0_f64 - t335 * t338 * t3200 * t4111 / 48.0_f64 + 7.0_f64 / 2304.0_f64 * t51807 - t53910 / 48.0_f64 - 119.0_f64 / 3456.0_f64 * t51819 + 7.0_f64 / 2304.0_f64 * t51827 - 7.0_f64 / 288.0_f64 * t51829 - t53925 / 6.0_f64 + t8629 * t52600 / 96.0_f64 + t53930 / 96.0_f64 - t53936 / 384.0_f64 - t8793 * t52241 / 16.0_f64 - t55695 + t4385 * t55698 / 96.0_f64 - t55702;
    t55703
}
