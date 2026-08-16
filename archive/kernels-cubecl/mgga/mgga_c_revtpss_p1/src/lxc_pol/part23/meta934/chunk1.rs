//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3072/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3072<F: Float>(t20892: F, t5192: F, t45000: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F) -> (F, F) {
    let t81266 = F::cast_from(0.30762056574649219972e4_f64) * t5192 * t20892;
    let t81286 = F::cast_from(0.23744444444444444444e-1_f64) * t68255 - F::cast_from(0.15829629629629629629e-1_f64) * t68257 + F::cast_from(0.11872222222222222222e-1_f64) * t81156 - F::cast_from(0.35616666666666666667e-1_f64) * t81158 + F::cast_from(0.59361111111111111111e-1_f64) * t81162 + F::cast_from(0.23744444444444444444e0_f64) * t81167 + t45000 - F::cast_from(0.2137e0_f64) * t81171 - F::cast_from(0.42739999999999999999e0_f64) * t81175 - F::cast_from(0.35616666666666666666e-1_f64) * t81179 - F::cast_from(0.11872222222222222222e-1_f64) * t81184 - F::cast_from(0.35616666666666666666e-1_f64) * t81188 + F::cast_from(0.32055e0_f64) * t81192 + F::cast_from(0.4274e0_f64) * t81196 + F::cast_from(0.10685e0_f64) * t81200 + F::cast_from(0.10685e0_f64) * t81204 + F::cast_from(0.35616666666666666666e-1_f64) * t81209 - F::cast_from(0.52765432098765432099e-1_f64) * t81214 - F::cast_from(0.19787037037037037037e-1_f64) * t68262 - F::cast_from(0.35616666666666666666e-1_f64) * t68277;
    (t81266, t81286)
}
