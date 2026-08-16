//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1240/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1240(t13780: f64, t13859: f64, t3990: f64, t9326: f64, t14664: f64, t9270: f64, t14705: f64, t51666: f64, t14637: f64, t3974: f64, t8759: f64, t11375: f64, t1185: f64, t13924: f64, t50995: f64, t51053: f64, t51675: f64, t53134: f64, t53140: f64, t53152: f64, t53155: f64, t53158: f64, t53166: f64, t53170: f64, t8776: f64, t9697: f64) -> f64 {
    let t53174 = t13859 * t3990 * t13780 * t9326;
    let t53177 = 7.0_f64 / 72.0_f64 * t9270 * t14664;
    let t53178 = t51666 * t14705;
    let t53179 = 7.0_f64 / 576.0_f64 * t53178;
    let t53182 = t14637 * t3990 * t3974 * t8759;
    let t53184 = t53134 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t50995 - t53140 / 384.0_f64 + t8776 * t1185 * t13924 / 32.0_f64 - t9697 * t1185 * t51053 / 32.0_f64 - t11375 * t51675 / 48.0_f64 + t53152 / 384.0_f64 - t53155 - t53158 / 96.0_f64 - t53166 / 384.0_f64 + t53170 / 384.0_f64 + t53174 / 768.0_f64 - t53177 - t53179 + 5.0_f64 / 768.0_f64 * t53182;
    t53184
}
