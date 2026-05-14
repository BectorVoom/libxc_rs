//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1364/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1364<F: Float>(t10865: F, t10868: F, t10872: F, t10970: F, t10973: F, t10976: F, t10980: F, t10983: F, t10987: F, t14891: F, t14895: F, t14899: F, t14903: F, t14906: F, t14911: F, t169: F, t19005: F, t2675: F, t2676: F, t299: F, t301: F, t5670: F, t777: F, t9126: F) -> (F,) {
    let t19828 = -t777 * t9126 * t2675 - t5670 * t2676 - 0.0005811348303577384 * t10865 - 0.0017434044910732151 * t10868 - t10872 - 0.02394846802050922 * t14891 - 0.04789693604101844 * t14895 - 0.02394846802050922 * t14899 - 7.28743077268604e-05 * t14903 - 0.00010931146159029059 * t14906 - 9.138438188948293e-06 * t14911 + 0.020267214298646783 * t169 * t299 * t19005 * t301 - t10970 - 1.82185769317151e-05 * t10973 - 0.00010931146159029059 * t10976 - t10980 + 0.0003279343847708718 * t10983 + t10987;
    (t19828,)
}
