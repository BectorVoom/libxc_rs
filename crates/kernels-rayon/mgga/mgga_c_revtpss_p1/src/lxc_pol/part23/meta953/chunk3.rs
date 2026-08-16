//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3166/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3166(t44307: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> f64 {
    let t83211 = 0.11111111111111111111e-1_f64 * t68255 - 0.74074074074074074073e-2_f64 * t68257 + 0.55555555555555555553e-2_f64 * t81156 - 0.16666666666666666667e-1_f64 * t81158 + 0.27777777777777777778e-1_f64 * t81162 + 0.11111111111111111111e0_f64 * t81167 + t44307 - 0.99999999999999999999e-1_f64 * t81171 - 0.19999999999999999999e0_f64 * t81175 - 0.16666666666666666666e-1_f64 * t81179 - 0.55555555555555555555e-2_f64 * t81184 - 0.16666666666666666666e-1_f64 * t81188 + 0.15e0_f64 * t81192 + 0.2e0_f64 * t81196 + 0.50000000000000000001e-1_f64 * t81200 + 0.50000000000000000001e-1_f64 * t81204 + 0.16666666666666666667e-1_f64 * t81209 - 0.24691358024691358025e-1_f64 * t81214 - 0.92592592592592592592e-2_f64 * t68262 - 0.16666666666666666667e-1_f64 * t68277;
    t83211
}
