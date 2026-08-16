//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3205/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3205(t44865: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> f64 {
    let t84156 = 0.19755555555555555556e-1_f64 * t68255 - 0.13170370370370370371e-1_f64 * t68257 + 0.9877777777777777778e-2_f64 * t81156 - 0.29633333333333333334e-1_f64 * t81158 + 0.49388888888888888889e-1_f64 * t81162 + 0.19755555555555555556e0_f64 * t81167 + t44865 - 0.1778e0_f64 * t81171 - 0.35560000000000000001e0_f64 * t81175 - 0.29633333333333333334e-1_f64 * t81179 - 0.9877777777777777778e-2_f64 * t81184 - 0.29633333333333333334e-1_f64 * t81188 + 0.26670000000000000001e0_f64 * t81192 + 0.35560000000000000001e0_f64 * t81196 + 0.88900000000000000002e-1_f64 * t81200 + 0.88900000000000000002e-1_f64 * t81204 + 0.29633333333333333334e-1_f64 * t81209 - 0.43901234567901234568e-1_f64 * t81214 - 0.16462962962962962963e-1_f64 * t68262 - 0.29633333333333333334e-1_f64 * t68277;
    t84156
}
