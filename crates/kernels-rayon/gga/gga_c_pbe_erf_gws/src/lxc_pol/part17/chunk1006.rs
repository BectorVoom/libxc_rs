//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1006/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1006(t1114: f64, t6644: f64, t6648: f64, t3134: f64, t6217: f64, t3148: f64, t6484: f64, t6485: f64, t4341: f64, t4349: f64, t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64, t6907: f64, t6911: f64, t6918: f64, t6923: f64, t6929: f64, t6932: f64, t6966: f64, t6969: f64, t7984: f64, t8517: f64, t8521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9035 = t1114 * t6644;
    let t9037 = t9035 * t6648 / 48.0_f64;
    let t9039 = t6217 * t3134 / 96.0_f64;
    let t9041 = 7.0_f64 / 72.0_f64 * t6484 * t3148;
    let t9042 = 7.0_f64 / 72.0_f64 * t6485;
    let t9043 = t6907 + t4341 - t6911 - t4349 + t6918 + t4503 - t4506 - t4513 + t4539 - t6923 + t4542 - t6929 + t6932 + t6966 + t6969 - t8517 - t7984 - t8521;
    (t9035, t9037, t9039, t9041, t9042, t9043)
}
