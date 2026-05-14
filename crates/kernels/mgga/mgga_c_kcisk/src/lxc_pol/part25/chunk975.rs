//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 975/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk975<F: Float>(t17300: F, t7256: F, t11179: F, t6763: F, t5015: F, t6758: F, t10802: F, t2448: F, t3934: F, t654: F, t10817: F, t10866: F, t10869: F, t10881: F, t10884: F, t10888: F, t17169: F, t17271: F, t17277: F, t17280: F, t17283: F, t17290: F, t17295: F, t1773: F, t1787: F, t2466: F, t5013: F, t5017: F, t5034: F, t5040: F, t7208: F, t7219: F) -> (F, F, F, F) {
    let t17301 = t7256 * t17300;
    let t17302 = t11179 * t17301;
    let t17305 = t6763 * t17300;
    let t17306 = t5015 * t17305;
    let t17309 = t6758 * t17300;
    let t17310 = t10802 * t17309;
    let t17317 = t2448 * t654 * t3934;
    let t17320 = 0.2398771828823642295e-1 * t10866 - 0.17990788716177317213e-1 * t10869 - 0.5397236614853195164e-1 * t1773 * t17271 - 0.28785261945883707542e0 * t7219 * t5034 + 0.11993859144118211475e-1 * t17277 - t17280 - 0.35981577432354634426e-1 * t1773 * t17283 + 0.28785261945883707542e0 * t17169 * t1787 + 0.14392630972941853771e0 * t7219 * t5040 - 0.10794473229706390328e0 * t17290 * t1787 - t17295 - 0.5397236614853195164e-1 * t10817 * t2466 - 0.5397236614853195164e-1 * t7208 * t5040 + 0.71963154864709268853e-1 * t5013 * t17302 + 0.71963154864709268852e-1 * t5013 * t17306 - 0.47975436576472845902e-1 * t5013 * t17310 - 0.79959060960788076501e-2 * t10881 + 0.59969295720591057377e-2 * t10884 + 0.79959060960788076502e-2 * t10888 - 0.35981577432354634426e-1 * t17317 * t5017;
    (t17301, t17305, t17309, t17320)
}
