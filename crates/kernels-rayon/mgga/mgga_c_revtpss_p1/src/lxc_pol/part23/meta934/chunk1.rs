//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3072/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3072(t20892: f64, t5192: f64, t45000: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> (f64, f64) {
    let t81266 = 0.30762056574649219972e4_f64 * t5192 * t20892;
    let t81286 = 0.23744444444444444444e-1_f64 * t68255 - 0.15829629629629629629e-1_f64 * t68257 + 0.11872222222222222222e-1_f64 * t81156 - 0.35616666666666666667e-1_f64 * t81158 + 0.59361111111111111111e-1_f64 * t81162 + 0.23744444444444444444e0_f64 * t81167 + t45000 - 0.2137e0_f64 * t81171 - 0.42739999999999999999e0_f64 * t81175 - 0.35616666666666666666e-1_f64 * t81179 - 0.11872222222222222222e-1_f64 * t81184 - 0.35616666666666666666e-1_f64 * t81188 + 0.32055e0_f64 * t81192 + 0.4274e0_f64 * t81196 + 0.10685e0_f64 * t81200 + 0.10685e0_f64 * t81204 + 0.35616666666666666666e-1_f64 * t81209 - 0.52765432098765432099e-1_f64 * t81214 - 0.19787037037037037037e-1_f64 * t68262 - 0.35616666666666666666e-1_f64 * t68277;
    (t81266, t81286)
}
