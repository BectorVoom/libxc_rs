//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1185/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1185<F: Float>(t428: F, t929: F, t22541: F, t22572: F, t25699: F, t100838: F, t11090: F, t11121: F, t11147: F, t11150: F, t1603: F, t1691: F, t22565: F, t22568: F, t22597: F, t22602: F, t22603: F, t22777: F, t25625: F, t25649: F, t25658: F, t25734: F, t25752: F, t25756: F, t25774: F, t25802: F, t25820: F, t3067: F, t34433: F, t420: F, t44965: F, t45082: F, t45526: F, t45574: F, t5537: F, t5538: F, t5540: F, t5546: F, t6427: F, t7839: F, t92336: F, t92341: F, t92666: F, t92786: F, t93034: F, t93084: F) -> (F, F) {
    let t101333 = t929 * t428;
    let t101360 = t22541 * t22572 * t25699;
    let t101374 = -0.27568129967481981592e-3 * t22565 * t25658 * t7839 + 0.64109413167231678973e-5 * t44965 * t25625 * t7839 - 0.27568129967481981592e-3 * t92336 * t25820 - 0.27568129967481981592e-3 * t92341 * t25820 + 0.31073410497668637766e-5 * t45526 * t25752 * t25756 - 0.24041029937711879616e-5 * t44965 * t5537 * t5546 * t45574 + 0.12020514968855939808e-5 * t11121 * t22602 * t5546 * t101333 + 0.87299078230359608375e-3 * t22565 * t6427 * t45082 - 0.55136259934963963188e-3 * t22597 * t22777 * t25649 - 0.25845121844514357744e-4 * t22603 * t5540 * t100838 + 0.55136259934963963188e-4 * t5538 * t22777 * t25802 + 0.18366082263971467211e-4 * t5538 * t92786 * t25774 - 0.23254900946437792e-1 * t1603 * t93084 * t6427 + 0.68099848938271604939e-1 * t22541 * t22568 * t25699 - 0.85124811172839506173e-2 * t101360 + 0.77462893625097599762e-3 * t25734 * t11090 + 0.38731446812548799881e-3 * t25734 * t11147 - 0.64507906339763927061e-5 * t25734 * t11150 + 0.46509801892875584e-1 * t92666 * t3067 + 0.20834636627556862177e-5 * t93034 * t420 * t34433 * t1691;
    (t101333, t101374)
}
