//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 616/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk616<F: Float>(t169: F, t174: F, t1650: F, t2641: F, t167: F, t176: F, t740: F, t833: F, t44: F, t4517: F, t230: F, t1655: F, t908: F, t1659: F, t911: F, t2633: F, t234: F, t441: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t4518 = t2641 * t1650;
    let t4521 = t176 * t167;
    let t4525 = piecewise3(t175, 0.0, 4.0 / 9.0 * t4518 * t833 - 8.0 / 3.0 * t4521 * t740);
    let t4527 = (t4517 + t4525) * t44;
    let t4528 = t4527 * t230;
    let t4529 = t1655 * t908;
    let t4530 = t911 * t1659;
    let t4532 = 2.0 * t2633;
    let t4533 = piecewise3(t170, 0.0, t4532);
    let t4534 = t234 * t4533;
    let t4535 = t4534 * t441;
    (t4518, t4527, t4528, t4529, t4530, t4532, t4534, t4535)
}
