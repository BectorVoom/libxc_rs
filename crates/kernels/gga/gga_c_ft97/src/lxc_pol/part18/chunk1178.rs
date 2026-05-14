//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1178/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1178<F: Float>(t1594: F, t929: F, t1527: F, t1691: F, t925: F, t397: F, t938: F, t22511: F, t3076: F, t32167: F, t100634: F, t11604: F, t1737: F, t100992: F, t11205: F, t11340: F, t11348: F, t11353: F, t11368: F, t1625: F, t1685: F, t1689: F, t1697: F, t1751: F, t22515: F, t22522: F, t22568: F, t22590: F, t22591: F, t22603: F, t22777: F, t22820: F, t22826: F, t22842: F, t25679: F, t25692: F, t25698: F, t25703: F, t25708: F, t25760: F, t25779: F, t3061: F, t3066: F, t3067: F, t34433: F, t39: F, t401: F, t428: F, t5569: F, t5570: F, t5571: F, t7889: F, t92622: F, t92689: F, t93122: F, t93136: F) -> (F, F, F) {
    let t101026 = t1594 * t929;
    let t101031 = t1527 * t925 * t1691;
    let t101047 = t397 * t938;
    let t101075 = t3076 * t32167 * t22511;
    let t101089 = t100634 * t1737 * t11604;
    let t101096 = -0.77462893625097599762e-3 * t22826 * t101026 * t1625 - 0.34724394379261436962e-6 * t92622 * t101031 - 0.44455354858818847408e-2 * t7889 * t22591 * t25679 * t1751 - 0.28107073075534343171e-3 * t22842 * t938 * t39 * t1689 * t1697 + 0.44455354858818847408e-2 * t22590 * t22591 * t25679 * t1685 - 0.47419045182740103901e-1 * t22590 * t22591 * t101047 * t401 + 0.47419045182740103901e-1 * t7889 * t22591 * t101047 * t428 + 0.27568129967481981593e-3 * t22603 * t22777 * t25779 + 0.46509801892875584e-1 * t22826 * t11340 + 0.23254900946437792e-1 * t22826 * t11368 + 0.46509801892875584e-1 * t92689 * t3067 + 0.46509801892875584e-1 * t22826 * t11348 + 0.23254900946437792e-1 * t22826 * t11353 + 0.77462893625097599762e-3 * t100992 * t3061 - 0.68099848938271604939e-1 * t22522 * t22568 * t25760 - 0.54493253106890798149e-2 * t101075 * t22515 * t34433 * t22820 + 0.29693535778629056444e-3 * t93136 * t25692 * t25698 * t3066 - 0.29693535778629056444e-3 * t93122 * t25692 * t25703 * t3066 - 0.3404992446913580247e-1 * t25708 * t101089 + 0.22270151833971792333e-3 * t5569 * t5570 * t5571 * t11205;
    (t101031, t101089, t101096)
}
