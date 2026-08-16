//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3110/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3110(t43771: f64, t43814: f64, t43817: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81399: f64, t81401: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t81416: f64) -> (f64, f64) {
    let t81904 = 0.40256666666666666668e0_f64 * t68255 - 0.26837777777777777777e0_f64 * t68257 - 0.24528888888888888889e0_f64 * t43771 + 0.20128333333333333333e0_f64 * t81156 - 0.60385e0_f64 * t81158 + 0.10064166666666666667e1_f64 * t81162 + 0.40256666666666666666e1_f64 * t81167 + 0.258925e1_f64 * t81399 + t43814 + t43817 + 0.16504875e0_f64 * t81401;
    let t81917 = -0.36231e1_f64 * t81171 - 0.72462e1_f64 * t81175 - 0.60384999999999999999e0_f64 * t81179 - 0.20128333333333333333e0_f64 * t81184 - 0.60384999999999999999e0_f64 * t81188 + 0.543465e1_f64 * t81192 + 0.72462e1_f64 * t81196 + 0.181155e1_f64 * t81200 + 0.181155e1_f64 * t81204 + 0.60385e0_f64 * t81209 - 0.89459259259259259259e0_f64 * t81214 + 0.11038e0_f64 * t81416;
    (t81904, t81917)
}
