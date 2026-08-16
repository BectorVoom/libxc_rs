//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1164/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1164(t14716: f64, t4213: f64, t840: f64, t14745: f64, t4230: f64, t14752: f64, t13974: f64, t14333: f64, t14722: f64, t14727: f64, t14729: f64, t14731: f64, t14734: f64, t14737: f64) -> (f64, f64, f64) {
    let t14986 = 7.0_f64 / 1152.0_f64 * t14716;
    let t14989 = t840 * t4213;
    let t14996 = 7.0_f64 / 72.0_f64 * t14745;
    let t14997 = t840 * t4230;
    let t14999 = 7.0_f64 / 144.0_f64 * t14752;
    let t15000 = -t14986 - t14722 / 1536.0_f64 + t14727 / 1536.0_f64 + 7.0_f64 / 288.0_f64 * t14989 + 7.0_f64 / 288.0_f64 * t14333 + t14729 / 24.0_f64 + t14731 / 8.0_f64 - t14734 / 48.0_f64 + t13974 + t14737 / 48.0_f64 + t14996 + 7.0_f64 / 288.0_f64 * t14997 + t14999;
    (t14989, t14997, t15000)
}
