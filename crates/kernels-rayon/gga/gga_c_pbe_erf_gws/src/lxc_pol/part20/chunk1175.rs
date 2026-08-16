//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1175/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1175(t14752: f64, t14506: f64, t14520: f64, t14551: f64, t14554: f64, t14558: f64, t14563: f64, t3703: f64, t3944: f64, t1105: f64, t14390: f64, t1167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14999 = 7.0_f64 / 144.0_f64 * t14752;
    let t15050 = 7.0_f64 / 576.0_f64 * t14506;
    let t15057 = 7.0_f64 / 144.0_f64 * t14520;
    let t15070 = 7.0_f64 / 576.0_f64 * t14551;
    let t15072 = 7.0_f64 / 144.0_f64 * t14554;
    let t15074 = 7.0_f64 / 288.0_f64 * t14558;
    let t15076 = 7.0_f64 / 72.0_f64 * t14563;
    let t15118 = t3944 * t3703;
    let t15121 = t14390 * t1105;
    let t15124 = t1105 * t1167;
    (t14999, t15050, t15057, t15070, t15072, t15074, t15076, t15118, t15121, t15124)
}
