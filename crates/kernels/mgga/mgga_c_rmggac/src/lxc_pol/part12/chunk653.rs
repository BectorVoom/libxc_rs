//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 653/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk653<F: Float>(t9126: F, t9129: F, t9148: F, t9223: F, t9225: F, t9229: F, t8328: F, t8331: F, t8334: F, t8350: F, t8356: F, t8467: F, t8470: F, t8477: F, t8484: F, t8488: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9646 = 0.2993560425465952141e-1 * t9126;
    let t9647 = 0.5987120850931904282e-1 * t9129;
    let t9653 = 0.1064114997332445985e-4 * t9148;
    let t9670 = 0.1064114997332445985e-4 * t9223;
    let t9671 = 0.8980681276397856423e-1 * t9225;
    let t9672 = 0.5987120850931904282e-1 * t9229;
    let t9716 = 0.19211284388664477842e-2 * t8328;
    let t9717 = 0.81300399444200075504e-3 * t8331;
    let t9718 = 0.81300399444200075504e-3 * t8334;
    let t9729 = 0.30487649791575028314e-3 * t8350;
    let t9730 = 0.30487649791575028314e-3 * t8356;
    let t9743 = 0.72042316457491791906e-3 * t8467;
    let t9744 = 0.10248087766267884742e-3 * t8470;
    let t9758 = 0.30487649791575028314e-3 * t8477;
    let t9759 = 0.43368970657079495312e-4 * t8484;
    let t9760 = 0.30487649791575028314e-3 * t8488;
    (t9646, t9647, t9653, t9670, t9671, t9672, t9716, t9717, t9718, t9729, t9730, t9743, t9744, t9758, t9759, t9760)
}
