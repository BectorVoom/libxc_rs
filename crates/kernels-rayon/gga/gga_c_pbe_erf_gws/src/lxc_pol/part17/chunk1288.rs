//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1288/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1288(t1144: f64, t13923: f64, t859: f64, t13911: f64, t26958: f64, t13917: f64, t53156: f64, t9333: f64, t22336: f64, t4002: f64, t14667: f64, t22263: f64, t2409: f64, t3066: f64, t4385: f64, t51569: f64, t51815: f64, t51825: f64, t51827: f64, t51829: f64, t53915: f64, t53925: f64, t53930: f64, t53936: f64, t8734: f64, t8793: f64) -> f64 {
    let t53939 = t859 * t1144 * t13923;
    let t53943 = 7.0_f64 / 72.0_f64 * t26958 * t13911;
    let t53945 = t13917 * t53156 * t9333;
    let t53948 = 7.0_f64 / 144.0_f64 * t22336 * t4002;
    let t53949 = -7.0_f64 / 72.0_f64 * t51815 - t53915 + 35.0_f64 / 108.0_f64 * t51825 + 7.0_f64 / 4608.0_f64 * t51827 + t3066 * t2409 * t8734 * t14667 / 24.0_f64 - 7.0_f64 / 576.0_f64 * t51829 - t53925 / 12.0_f64 - t8793 * t51569 / 16.0_f64 + t53930 / 192.0_f64 - t22263 * t4002 / 48.0_f64 - t53936 / 768.0_f64 + t4385 * t53939 / 96.0_f64 - t53943 + t53945 / 256.0_f64 + t53948;
    t53949
}
