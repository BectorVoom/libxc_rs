//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1307/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1307(t51312: f64, t9035: f64, t14570: f64, t6538: f64, t3123: f64, t51430: f64, t14007: f64, t9438: f64, t51252: f64, t54133: f64, t54136: f64, t54137: f64, t54139: f64, t54142: f64, t54144: f64, t54146: f64) -> f64 {
    let t54148 = t9035 * t51312;
    let t54150 = t6538 * t14570;
    let t54152 = t3123 * t51430;
    let t54153 = 7.0_f64 / 144.0_f64 * t54152;
    let t54154 = t14007 * t9438;
    let t54156 = t54133 / 16.0_f64 - t54136 + t54137 / 256.0_f64 + 3.0_f64 / 256.0_f64 * t54139 - 7.0_f64 / 288.0_f64 * t51252 + t54142 / 96.0_f64 - t54144 / 384.0_f64 - t54146 / 96.0_f64 + t54148 / 48.0_f64 - t54150 / 96.0_f64 + t54153 - t54154 / 384.0_f64;
    t54156
}
