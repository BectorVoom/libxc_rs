//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1153/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1153(t13792: f64, t14617: f64, t14479: f64, t14579: f64, t14585: f64, t14589: f64, t14593: f64, t14597: f64, t14599: f64, t14603: f64, t14605: f64, t14609: f64, t14611: f64, t14615: f64, t335: f64) -> f64 {
    let t14618 = t13792 * t14617;
    let t14620 = t14479 / 96.0_f64 - t335 * t14579 / 96.0_f64 - t14585 / 1536.0_f64 - t14589 / 1536.0_f64 - t14593 / 384.0_f64 - t14597 / 1536.0_f64 - 7.0_f64 / 144.0_f64 * t14599 + t14603 / 768.0_f64 + 7.0_f64 / 4608.0_f64 * t14605 - t14609 / 3072.0_f64 + 7.0_f64 / 4608.0_f64 * t14611 - t14615 / 768.0_f64 - t14618 / 96.0_f64;
    t14620
}
