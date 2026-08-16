//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1308/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1308(t13776: f64, t3975: f64, t46392: f64, t13781: f64, t3222: f64, t3886: f64, t3972: f64, t14733: f64, t8700: f64, t14113: f64, t15204: f64, t1144: f64, t12201: f64, t13772: f64, t13939: f64, t14577: f64, t335: f64, t338: f64, t3913: f64, t3917: f64, t3921: f64, t4002: f64, t51530: f64, t53646: f64, t53656: f64, t56697: f64, t56701: f64, t56703: f64, t56708: f64, param_a_c: f64) -> f64 {
    let t56717 = t13776 * t3975 * t46392;
    let t56722 = t3972 * t13781 * t3886 * param_a_c * t3222;
    let t56724 = t14733 * t8700;
    let t56728 = t14113 * t15204;
    let t56735 = -t56697 / 1536.0_f64 - t56701 / 3072.0_f64 - t53646 + 7.0_f64 / 144.0_f64 * t56703 - t56708 / 384.0_f64 - t12201 * t4002 / 96.0_f64 - t3913 * t13772 / 96.0_f64 - t3917 * t13939 / 96.0_f64 - t56717 / 384.0_f64 - t56722 / 1536.0_f64 + t56724 / 24.0_f64 - t3921 * t13939 / 96.0_f64 + t53656 + 7.0_f64 / 1152.0_f64 * t56728 - 119.0_f64 / 3456.0_f64 * t51530 - t335 * t338 * t1144 * t14577 / 48.0_f64;
    t56735
}
