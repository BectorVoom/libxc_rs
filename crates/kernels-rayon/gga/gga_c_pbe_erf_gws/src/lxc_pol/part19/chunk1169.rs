//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1169/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1169(t14520: f64, t14030: f64, t14508: f64, t14510: f64, t14512: f64, t14514: f64, t14516: f64, t14518: f64, t14523: f64, t14525: f64, t15050: f64, t14551: f64) -> (f64, f64) {
    let t15057 = 7.0_f64 / 144.0_f64 * t14520;
    let t15060 = -t14030 + t15050 - t14508 / 48.0_f64 + t14510 / 24.0_f64 + t14512 / 24.0_f64 + t14514 / 24.0_f64 + 5.0_f64 / 192.0_f64 * t14516 + t14518 / 96.0_f64 - t15057 - t14523 / 48.0_f64 + t14525 / 192.0_f64;
    let t15070 = 7.0_f64 / 576.0_f64 * t14551;
    (t15060, t15070)
}
