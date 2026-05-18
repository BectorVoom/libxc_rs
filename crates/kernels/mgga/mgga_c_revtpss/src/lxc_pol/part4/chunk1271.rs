//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1271/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1271<F: Float>(t1659: F, t3230: F, t1660: F, t3201: F, t1058: F, t4798: F, t1053: F, t4797: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11890: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F, F) {
    let t15859 = t1659 * t3230;
    let t15862 = t1660 * t3201;
    let t15865 = F::new(0.28582678745379824648e-3) * t4798 * t1058;
    let t15866 = t4797 * t1053;
    let t15874 = F::new(0.37037037037037037037e-2) * t15127;
    let t15875 = F::new(0.11111111111111111111e-1) * t15125;
    let t15876 = F::new(0.55555555555555555556e-2) * t15191;
    let t15885 = -t11890 - F::new(0.74074074074074074074e-2) * t11134 + F::new(0.18518518518518518519e-2) * t11136 - F::new(0.55555555555555555556e-2) * t11138 + F::new(0.27777777777777777778e-2) * t11140 - F::new(0.37037037037037037037e-2) * t15189 + t15874 - t15875 + t15876 - F::new(0.92592592592592592592e-2) * t15142 + F::new(0.33333333333333333333e-1) * t15156 - F::new(0.11111111111111111111e-1) * t15132 - F::new(0.55555555555555555555e-2) * t15137 - F::new(0.50000000000000000001e-1) * t15160 + F::new(0.33333333333333333334e-1) * t15147 + F::new(0.16666666666666666667e-1) * t15151 - F::new(0.83333333333333333333e-2) * t15195;
    (t15859, t15862, t15865, t15866, t15885)
}
