//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 460/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk460(t562: f64, t572: f64, t418: f64, t1821: f64, t1820: f64, t590: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t1822 = t562 * t572;
    let t1823 = t1822 * t418;
    let t1824 = t1821 * t1823;
    let t1826 = 16.0_f64 / 45.0_f64 * t1820 * t1824;
    let t1827 = t590 * t597;
    (t1823, t1824, t1826, t1827)
}
