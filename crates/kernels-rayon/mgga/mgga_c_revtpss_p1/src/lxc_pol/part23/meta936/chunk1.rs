//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3077/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3077(t24324: f64, t3379: f64, t43881: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> (f64, f64) {
    let t81352 = 1.0_f64 * t3379 * t24324;
    let t81379 = 4.0_f64 / 9.0_f64 * t68255 - 8.0_f64 / 27.0_f64 * t68257 + 2.0_f64 / 9.0_f64 * t81156 - 2.0_f64 / 3.0_f64 * t81158 + 10.0_f64 / 9.0_f64 * t81162 + 40.0_f64 / 9.0_f64 * t81167 + t43881 - 4.0_f64 * t81171 - 8.0_f64 * t81175 - 2.0_f64 / 3.0_f64 * t81179 - 2.0_f64 / 9.0_f64 * t81184 - 2.0_f64 / 3.0_f64 * t81188 + 6.0_f64 * t81192 + 8.0_f64 * t81196 + 2.0_f64 * t81200 + 2.0_f64 * t81204 + 2.0_f64 / 3.0_f64 * t81209 - 80.0_f64 / 81.0_f64 * t81214 - 10.0_f64 / 27.0_f64 * t68262 - 2.0_f64 / 3.0_f64 * t68277;
    (t81352, t81379)
}
