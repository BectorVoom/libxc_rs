//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3061/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3061(t43995: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> f64 {
    let t81218 = 0.12361111111111111111e-1_f64 * t68255 - 0.82407407407407407408e-2_f64 * t68257 + 0.61805555555555555553e-2_f64 * t81156 - 0.18541666666666666667e-1_f64 * t81158 + 0.30902777777777777778e-1_f64 * t81162 + 0.12361111111111111111e0_f64 * t81167 + t43995 - 0.11125e0_f64 * t81171 - 0.22249999999999999999e0_f64 * t81175 - 0.18541666666666666666e-1_f64 * t81179 - 0.61805555555555555555e-2_f64 * t81184 - 0.18541666666666666666e-1_f64 * t81188 + 0.166875e0_f64 * t81192 + 0.2225e0_f64 * t81196 + 0.55625000000000000001e-1_f64 * t81200 + 0.55625000000000000001e-1_f64 * t81204 + 0.18541666666666666667e-1_f64 * t81209 - 0.27469135802469135803e-1_f64 * t81214 - 0.10300925925925925926e-1_f64 * t68262 - 0.18541666666666666667e-1_f64 * t68277;
    t81218
}
