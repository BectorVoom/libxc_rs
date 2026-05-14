//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 542/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk542<F: Float>(t1169: F, t5142: F, t1744: F, t3479: F, t1168: F, t3358: F, t3483: F, t5044: F, t5049: F, t5054: F, t5058: F, t448: F, t1179: F, t1749: F, t1187: F, t1757: F) -> (F, F, F, F, F) {
    let t5143 = t5142 * t1169;
    let t5146 = t1744 * t3479;
    let t5147 = t5146 * t1168;
    let t5155 = t3483 - 0.30902777777777777778e-2 * t3358 - 0.30902777777777777778e-2 * t5044 - 0.61805555555555555555e-2 * t5049 + 0.18541666666666666667e-1 * t5054 + 0.92708333333333333333e-2 * t5058;
    let t5156 = t5155 * t448;
    let t5158 = t1749 * t1179;
    let t5163 = t1757 * t1187;
    (t5143, t5147, t5156, t5158, t5163)
}
