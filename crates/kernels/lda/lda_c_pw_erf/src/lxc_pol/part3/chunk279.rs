//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 279/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk279<F: Float>(t159: F, t285: F, t780: F, t171: F, t125: F, t145: F, t153: F, t156: F, t163: F, t164: F, t168: F, t169: F, t242: F, t245: F, t279: F, t281: F, t296: F, t299: F, t301: F, t405: F, t456: F, t460: F, t481: F, t487: F, t491: F, t624: F, t634: F, t638: F, t677: F, t686: F, t694: F, t698: F, t702: F, t709: F, t711: F, t721: F, t757: F, t777: F, t781: F, t841: F, t861: F, t865: F) -> (F, F, F) {
    let t872 = t780 * t159 * t285;
    let t875 = t171 * t780;
    let t887 = F::new(3.0) * t405 * t757 + t777 * t456 + (t460 - F::cast_from(0.031505407223141116_f64) * t781 * t164 - t481 - t487 + t491 - F::cast_from(0.005388405304614574_f64) * t169 * t171 * t841 * t163) * t125 + (t624 - F::cast_from(0.0837628205355044_f64) * t781 * t242 - t634 + t638 - F::cast_from(0.011938374665504766_f64) * t168 * t245 * t861 - t677 + F::cast_from(0.42708890021612717_f64) * t153 * t156 * t865) * t279 + t686 - F::cast_from(0.01197423401025461_f64) * t281 * t872 - t694 - t698 + (t702 - F::cast_from(0.031835665774679375_f64) * t169 * t875 * t242 - t709 - t711 + F::cast_from(0.05332506774217938_f64) * t145 * t865) * t296 - t721 + F::cast_from(0.020267214298646783_f64) * t169 * t299 * t865 * t301;
    (t872, t875, t887)
}
