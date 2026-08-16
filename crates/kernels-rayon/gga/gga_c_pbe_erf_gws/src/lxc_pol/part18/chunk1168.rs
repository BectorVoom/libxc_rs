//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1168/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1168(t2409: f64, t9716: f64, t3959: f64, t13989: f64, t13999: f64, t14002: f64, t14114: f64, t14742: f64, t14745: f64, t14749: f64, t14752: f64, t14755: f64, t14759: f64, t14768: f64, t14770: f64, t2408: f64, t3066: f64, t335: f64) -> (f64, f64) {
    let t14772 = t2409 * t9716;
    let t14773 = t3959 * t14772;
    let t14775 = -t335 * t14742 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t14745 + t3066 * t14749 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t14752 + t14755 / 1536.0_f64 + t13989 + t2408 * t14759 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t13999 + 7.0_f64 / 144.0_f64 * t14002 + 7.0_f64 / 1152.0_f64 * t14114 + t14768 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t14770 - t14773 / 48.0_f64;
    (t14772, t14775)
}
