//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1065/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1065(t14981: f64, t5928: f64, t74850: f64, t74856: f64, t74870: f64, t74889: f64, t77242: f64, t77243: f64, t77244: f64, t77246: f64, t77247: f64, t77249: f64, t77250: f64, t77251: f64, t77252: f64, t77253: f64, t77254: f64, t77255: f64) -> f64 {
    let t80176 = -0.35038612185802734374e-6_f64 * t74850 + 0.87596530464506835936e-6_f64 * t74856 + t77242 - t77243 + t77244 - t77246 + t74870 + 0.39914139006212695214e-1_f64 * t5928 * t14981 + t77247 - 0.58171619854173713844e-5_f64 * t74889 + t77249 + t77250 + t77251 - t77252 - t77253 - t77254 + t77255;
    t80176
}
