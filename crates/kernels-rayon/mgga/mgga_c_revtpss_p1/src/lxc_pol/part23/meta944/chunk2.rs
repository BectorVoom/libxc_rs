//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3100/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3100(t43771: f64, t45106: f64, t45107: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81399: f64, t81401: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t81416: f64) -> (f64, f64) {
    let t81678 = 0.68863333333333333332e0_f64 * t68255 - 0.45908888888888888888e0_f64 * t68257 - 0.30872592592592592592e0_f64 * t43771 + 0.34431666666666666667e0_f64 * t81156 - 0.103295e1_f64 * t81158 + 0.17215833333333333334e1_f64 * t81162 + 0.68863333333333333334e1_f64 * t81167 + 0.3529725e1_f64 * t81399 + t45106 + t45107 + 0.6311625e0_f64 * t81401;
    let t81691 = -0.61977e1_f64 * t81171 - 0.123954e2_f64 * t81175 - 0.103295e1_f64 * t81179 - 0.34431666666666666667e0_f64 * t81184 - 0.103295e1_f64 * t81188 + 0.929655e1_f64 * t81192 + 0.123954e2_f64 * t81196 + 0.309885e1_f64 * t81200 + 0.309885e1_f64 * t81204 + 0.103295e1_f64 * t81209 - 0.15302962962962962963e1_f64 * t81214 + 0.13892666666666666667e0_f64 * t81416;
    (t81678, t81691)
}
