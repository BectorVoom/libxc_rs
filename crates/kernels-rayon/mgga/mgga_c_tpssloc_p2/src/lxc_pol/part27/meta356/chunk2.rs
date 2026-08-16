//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1476/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1476(t13076: f64, t13080: f64, t13084: f64, t13087: f64, t13173: f64, t13177: f64, t13182: f64, t13186: f64, t13190: f64, t13193: f64, t13198: f64, t13202: f64, t13204: f64, t13208: f64, t13210: f64, t2623: f64, t2643: f64, t2681: f64, t4167: f64, t4178: f64, t4257: f64, t787: f64, t817: f64, t831: f64, t843: f64, t9602: f64, t9604: f64) -> f64 {
    let t13213 = -t2643 * t13076 / 3072.0_f64 - 5.0_f64 / 768.0_f64 * t2643 * t13080 - t4178 * t13084 / 384.0_f64 - 35.0_f64 / 216.0_f64 * t13087 - 119.0_f64 / 1728.0_f64 * t9602 + 7.0_f64 / 1152.0_f64 * t9604 + 5.0_f64 / 384.0_f64 * t2623 * t4257 - t817 * t13173 / 3072.0_f64 - t13177 * t831 / 1536.0_f64 - t4167 * t2681 / 3072.0_f64 - 119.0_f64 / 13824.0_f64 * t13182 - 5.0_f64 / 128.0_f64 * t843 * t13186 - t13190 + 5.0_f64 / 384.0_f64 * t843 * t13193 + 5.0_f64 / 768.0_f64 * t843 * t13198 + t13202 - t787 * t13204 / 48.0_f64 - t13208 + t2643 * t13210 / 768.0_f64;
    t13213
}
