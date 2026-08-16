//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2609/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2609(t18231: f64, t3961: f64, t1222: f64, t22169: f64, t11539: f64, t1174: f64, t21745: f64, t11546: f64, t11692: f64, t1227: f64, t15654: f64, t18321: f64, t18342: f64, t19083: f64, t22284: f64, t3440: f64, t3578: f64, t45134: f64, t4582: f64, t4733: f64, t4987: f64, t4989: f64, t5005: f64, t5033: f64, t52893: f64, t52919: f64, t6230: f64, t70316: f64, t70330: f64, t70339: f64, t71133: f64, t71197: f64) -> (f64, f64) {
    let t72788 = t18231 * t3961;
    let t72798 = t22169 * t1222;
    let t72815 = t1174 * t11539 * t21745;
    let t72823 = 5.0_f64 / 768.0_f64 * t1227 * t4582 * t15654 * t70339 - t52893 * t3578 * t72788 / 256.0_f64 + t45134 * t22284 / 1536.0_f64 + t11692 * t3578 * t6230 * t4733 / 1536.0_f64 + 19.0_f64 / 864.0_f64 * t72798 + 5.0_f64 / 4608.0_f64 * t1227 * t4582 * t4987 * t70316 + 55.0_f64 / 15552.0_f64 * t1227 * t4582 * t52919 * t70330 - 5.0_f64 / 432.0_f64 * t19083 * t4989 + 5.0_f64 / 2304.0_f64 * t5005 * t18342 + 11.0_f64 / 81.0_f64 * t18321 * t5033 + t72815 / 216.0_f64 + t1174 * t3440 * t71197 / 6.0_f64 - 7.0_f64 / 54.0_f64 * t1174 * t11546 * t71133;
    (t72788, t72823)
}
