//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1317/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1317(t1659: f64, t3230: f64, t1660: f64, t3201: f64, t1058: f64, t4798: f64, t1053: f64, t4797: f64, t15127: f64, t15125: f64, t15191: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11890: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t15195: f64) -> (f64, f64, f64, f64, f64) {
    let t15859 = t1659 * t3230;
    let t15862 = t1660 * t3201;
    let t15865 = 0.28582678745379824648e-3_f64 * t4798 * t1058;
    let t15866 = t4797 * t1053;
    let t15874 = 0.37037037037037037037e-2_f64 * t15127;
    let t15875 = 0.11111111111111111111e-1_f64 * t15125;
    let t15876 = 0.55555555555555555556e-2_f64 * t15191;
    let t15885 = -t11890 - 0.74074074074074074074e-2_f64 * t11134 + 0.18518518518518518519e-2_f64 * t11136 - 0.55555555555555555556e-2_f64 * t11138 + 0.27777777777777777778e-2_f64 * t11140 - 0.37037037037037037037e-2_f64 * t15189 + t15874 - t15875 + t15876 - 0.92592592592592592592e-2_f64 * t15142 + 0.33333333333333333333e-1_f64 * t15156 - 0.11111111111111111111e-1_f64 * t15132 - 0.55555555555555555555e-2_f64 * t15137 - 0.50000000000000000001e-1_f64 * t15160 + 0.33333333333333333334e-1_f64 * t15147 + 0.16666666666666666667e-1_f64 * t15151 - 0.83333333333333333333e-2_f64 * t15195;
    (t15859, t15862, t15865, t15866, t15885)
}
