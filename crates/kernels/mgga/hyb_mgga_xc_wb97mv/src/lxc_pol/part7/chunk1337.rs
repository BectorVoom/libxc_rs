//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1337/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1337<F: Float>(t1153: F, t396: F, t535: F, t32675: F, t7899: F, t10029: F, t10084: F, t10143: F, t10152: F, t1117: F, t11991: F, t11995: F, t11999: F, t24237: F, t24605: F, t28165: F, t28634: F, t28638: F, t28644: F, t2885: F, t28974: F, t2901: F, t32639: F, t32643: F, t32655: F, t32658: F, t32670: F, t32674: F, t32676: F, t3718: F, t4600: F, t4608: F, t511: F, t7833: F, t9865: F, t9947: F) -> (F, F, F) {
    let t32679 = t1153 * t396;
    let t32680 = t535 * t32679;
    let t32681 = t32675 * t7899;
    let t32684 = 2.0 * t10029 * t4600 - t4608 * t2885 + 0.384e0 * t28974 * t32639 - 0.23466666666666666666e0 * t10084 * t32643 - 0.432e1 * t28634 * t11999 * t2901 - 0.192e1 * t28638 * t7833 * t11991 + 0.288e1 * t24237 * t7833 * t11995 - 0.12096e2 * t28165 * t32655 - 0.4032e1 * t24605 * t32658 + 0.5376e1 * t28644 * t32639 + 800.0 / 3.0 * t1117 * t3718 * t10152 - 1600.0 / 3.0 * t511 * t9947 * t10143 - 0.14222222222222222222e1 * t9865 * t32670 - 0.36864e-4 * t32674 * t32676 + 0.36864e-4 * t32680 * t32681;
    (t32679, t32681, t32684)
}
