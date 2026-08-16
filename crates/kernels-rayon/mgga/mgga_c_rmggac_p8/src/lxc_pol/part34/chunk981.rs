//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 981/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk981(t77259: f64, t71196: f64, t71204: f64, t74870: f64, t74889: f64, t74919: f64, t77246: f64, t77247: f64, t77249: f64, t77250: f64, t77251: f64, t77252: f64, t77253: f64, t77254: f64, t77255: f64, t77256: f64, t77258: f64) -> f64 {
    let t77260 = 0.19863479950205658386e-4_f64 * t77259;
    let t77261 = -t77246 + t74870 + t77247 - 0.58171619854173713846e-5_f64 * t74889 + t77249 + t77250 + t77251 - t77252 - t77253 - t77254 + t77255 - t77256 + t71196 + 0.24527028530061914063e-5_f64 * t74919 + t77258 + t71204 - t77260;
    t77261
}
