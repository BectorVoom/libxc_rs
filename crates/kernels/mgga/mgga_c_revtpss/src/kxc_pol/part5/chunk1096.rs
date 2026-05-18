//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1096/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1096<F: Float>(t15191: F, t4628: F, t698: F, t15127: F, t15125: F, t3014: F, t4707: F, t15168: F, t4682: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15192 = F::new(0.20128333333333333334e0) * t15191;
    let t15197 = t698 * t4628;
    let t15198 = F::new(0.11038e0) * t15197;
    let t15209 = F::new(4.0) / F::new(27.0) * t15127;
    let t15210 = F::new(4.0) / F::new(9.0) * t15125;
    let t15211 = F::new(2.0) / F::new(9.0) * t15191;
    let t15258 = t4707 * t3014;
    let t15301 = F::new(0.22954444444444444444e0) * t15127;
    let t15312 = F::new(0.27785333333333333334e0) * t15168;
    let t15322 = F::new(0.34431666666666666666e0) * t15191;
    let t15324 = F::new(0.13892666666666666667e0) * t15197;
    let t15343 = t4682 * t964;
    (t15192, t15197, t15198, t15209, t15210, t15211, t15258, t15301, t15312, t15322, t15324, t15343)
}
