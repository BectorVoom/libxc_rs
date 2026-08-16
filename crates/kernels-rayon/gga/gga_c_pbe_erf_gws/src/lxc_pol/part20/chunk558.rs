//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 558/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk558(t138: f64, t1572: f64, t1577: f64, t2900: f64, t2902: f64, t2905: f64, t2919: f64, t514: f64, t520: f64, t985: f64, t101: f64, t1076: f64, t169: f64, t301: f64, t784: f64) -> (f64, f64, f64) {
    let t2921 = t138 * t2900 - t1572 * t985 + 2.0_f64 * t1577 * t2905 - t2902 * t520 - t2919 * t514;
    let t2922 = t101 * t2921;
    let t2926 = t169 * t784 * t1076 * t301;
    (t2921, t2922, t2926)
}
