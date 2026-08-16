//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3081/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3081(t56176: f64, t56183: f64, t43830: f64, t43832: f64, t44865: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64) -> f64 {
    let t56343 = 0.13170370370370370371e-1_f64 * t56176;
    let t56345 = 0.39511111111111111112e-1_f64 * t56183;
    let t56354 = t44865 - 0.35560000000000000001e0_f64 * t56151 + 0.88900000000000000002e-1_f64 * t56155 + 0.2667e0_f64 * t56159 + 0.29633333333333333334e-1_f64 * t56163 + 0.35560000000000000001e0_f64 * t56167 - 0.29633333333333333334e-1_f64 * t43830 + 0.98777777777777777781e-2_f64 * t43832 - 0.43901234567901234568e-1_f64 * t56174 - t56343 + 0.19755555555555555556e0_f64 * t56181 + t56345 - 0.59266666666666666668e-1_f64 * t56185 - 0.29633333333333333334e-1_f64 * t56187 - 0.88900000000000000002e-1_f64 * t56189 - 0.29633333333333333334e-1_f64 * t56194 - 0.29633333333333333334e-1_f64 * t56198 - 0.1778e0_f64 * t56203 - 0.9877777777777777778e-2_f64 * t56207 + 0.19755555555555555556e-1_f64 * t56209;
    t56354
}
