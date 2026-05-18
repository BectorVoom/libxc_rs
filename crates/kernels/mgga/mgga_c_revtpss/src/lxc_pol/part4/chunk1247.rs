//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1247/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1247<F: Float>(t15474: F, t935: F, t915: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11560: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F) {
    let t15475 = t15474 * t935;
    let t15477 = F::new(1.0) * t915 * t15475;
    let t15483 = F::new(0.41203703703703703704e-2) * t15127;
    let t15484 = F::new(0.12361111111111111111e-1) * t15125;
    let t15485 = F::new(0.61805555555555555556e-2) * t15191;
    let t15494 = -t11560 - F::new(0.82407407407407407407e-2) * t11134 + F::new(0.20601851851851851852e-2) * t11136 - F::new(0.61805555555555555556e-2) * t11138 + F::new(0.30902777777777777778e-2) * t11140 - F::new(0.41203703703703703704e-2) * t15189 + t15483 - t15484 + t15485 - F::new(0.10300925925925925926e-1) * t15142 + F::new(0.37083333333333333333e-1) * t15156 - F::new(0.12361111111111111111e-1) * t15132 - F::new(0.61805555555555555555e-2) * t15137 - F::new(0.55625000000000000001e-1) * t15160 + F::new(0.37083333333333333334e-1) * t15147 + F::new(0.18541666666666666667e-1) * t15151 - F::new(0.92708333333333333333e-2) * t15195;
    (t15477, t15494)
}
