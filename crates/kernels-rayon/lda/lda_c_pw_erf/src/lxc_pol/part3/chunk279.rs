//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 279/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk279(t159: f64, t285: f64, t780: f64, t171: f64, t125: f64, t145: f64, t153: f64, t156: f64, t163: f64, t164: f64, t168: f64, t169: f64, t242: f64, t245: f64, t279: f64, t281: f64, t296: f64, t299: f64, t301: f64, t405: f64, t456: f64, t460: f64, t481: f64, t487: f64, t491: f64, t624: f64, t634: f64, t638: f64, t677: f64, t686: f64, t694: f64, t698: f64, t702: f64, t709: f64, t711: f64, t721: f64, t757: f64, t777: f64, t781: f64, t841: f64, t861: f64, t865: f64) -> (f64, f64, f64) {
    let t872 = t780 * t159 * t285;
    let t875 = t171 * t780;
    let t887 = 3.0_f64 * t405 * t757 + t777 * t456 + (t460 - 0.031505407223141116_f64 * t781 * t164 - t481 - t487 + t491 - 0.005388405304614574_f64 * t169 * t171 * t841 * t163) * t125 + (t624 - 0.0837628205355044_f64 * t781 * t242 - t634 + t638 - 0.011938374665504766_f64 * t168 * t245 * t861 - t677 + 0.42708890021612717_f64 * t153 * t156 * t865) * t279 + t686 - 0.01197423401025461_f64 * t281 * t872 - t694 - t698 + (t702 - 0.031835665774679375_f64 * t169 * t875 * t242 - t709 - t711 + 0.05332506774217938_f64 * t145 * t865) * t296 - t721 + 0.020267214298646783_f64 * t169 * t299 * t865 * t301;
    (t872, t875, t887)
}
