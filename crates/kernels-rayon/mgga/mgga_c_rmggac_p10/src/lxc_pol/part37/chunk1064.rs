//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1064/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1064(t15881: f64, t874: f64, t352: f64, t15888: f64, t275: f64, t1356: f64, t74800: f64, t74824: f64, t74830: f64, t74831: f64, t74842: f64, t74846: f64, t77222: f64, t77224: f64, t77225: f64, t77228: f64, t77229: f64, t77230: f64, t77231: f64, t77236: f64, t77237: f64) -> (f64, f64) {
    let t80162 = t874 * t15881;
    let t80163 = t80162 * t352;
    let t80167 = t275 * t15888;
    let t80170 = -t77222 + 0.39914139006212695214e-1_f64 * t1356 * t80163 + t74800 + t77224 + t77225 + t77228 + t77229 + t77230 + t77231 - t74824 + t74830 - 0.58171619854173713844e-5_f64 * t74831 + t80167 - t77236 + t77237 + 0.17519306092901367186e-5_f64 * t74842 + 0.35038612185802734374e-6_f64 * t74846;
    (t80163, t80170)
}
