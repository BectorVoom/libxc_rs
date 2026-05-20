//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1596/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1596<F: Float>(t1659: F, t3230: F, t1660: F, t3201: F, t1058: F, t4798: F, t1053: F, t4797: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11890: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F, F, F, F) {
    let t15859 = t1659 * t3230;
    let t15862 = t1660 * t3201;
    let t15865 = F::cast_from(0.28582678745379824648e-3_f64) * t4798 * t1058;
    let t15866 = t4797 * t1053;
    let t15874 = F::cast_from(0.37037037037037037037e-2_f64) * t15127;
    let t15875 = F::cast_from(0.11111111111111111111e-1_f64) * t15125;
    let t15876 = F::cast_from(0.55555555555555555556e-2_f64) * t15191;
    let t15885 = -t11890 - F::cast_from(0.74074074074074074074e-2_f64) * t11134 + F::cast_from(0.18518518518518518519e-2_f64) * t11136 - F::cast_from(0.55555555555555555556e-2_f64) * t11138 + F::cast_from(0.27777777777777777778e-2_f64) * t11140 - F::cast_from(0.37037037037037037037e-2_f64) * t15189 + t15874 - t15875 + t15876 - F::cast_from(0.92592592592592592592e-2_f64) * t15142 + F::cast_from(0.33333333333333333333e-1_f64) * t15156 - F::cast_from(0.11111111111111111111e-1_f64) * t15132 - F::cast_from(0.55555555555555555555e-2_f64) * t15137 - F::cast_from(0.50000000000000000001e-1_f64) * t15160 + F::cast_from(0.33333333333333333334e-1_f64) * t15147 + F::cast_from(0.16666666666666666667e-1_f64) * t15151 - F::cast_from(0.83333333333333333333e-2_f64) * t15195;
    (t15859, t15862, t15865, t15866, t15885)
}
