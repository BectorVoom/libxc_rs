//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3063/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3063(t43830: f64, t43832: f64, t43995: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56176: f64, t56181: f64, t56184: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64) -> f64 {
    let t56211 = t43995 - 0.22249999999999999999e0_f64 * t56151 + 0.55625000000000000001e-1_f64 * t56155 + 0.166875e0_f64 * t56159 + 0.18541666666666666667e-1_f64 * t56163 + 0.2225e0_f64 * t56167 - 0.18541666666666666667e-1_f64 * t43830 + 0.61805555555555555556e-2_f64 * t43832 - 0.27469135802469135803e-1_f64 * t56174 - 0.82407407407407407408e-2_f64 * t56176 + 0.12361111111111111111e0_f64 * t56181 + t56184 - 0.37083333333333333333e-1_f64 * t56185 - 0.18541666666666666667e-1_f64 * t56187 - 0.55625e-1_f64 * t56189 - 0.18541666666666666666e-1_f64 * t56194 - 0.18541666666666666666e-1_f64 * t56198 - 0.11125e0_f64 * t56203 - 0.61805555555555555555e-2_f64 * t56207 + 0.12361111111111111111e-1_f64 * t56209;
    t56211
}
