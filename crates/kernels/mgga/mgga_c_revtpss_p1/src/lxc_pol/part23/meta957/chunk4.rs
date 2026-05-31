//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3205/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3205<F: Float>(t44865: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F) -> F {
    let t84156 = F::cast_from(0.19755555555555555556e-1_f64) * t68255 - F::cast_from(0.13170370370370370371e-1_f64) * t68257 + F::cast_from(0.9877777777777777778e-2_f64) * t81156 - F::cast_from(0.29633333333333333334e-1_f64) * t81158 + F::cast_from(0.49388888888888888889e-1_f64) * t81162 + F::cast_from(0.19755555555555555556e0_f64) * t81167 + t44865 - F::cast_from(0.1778e0_f64) * t81171 - F::cast_from(0.35560000000000000001e0_f64) * t81175 - F::cast_from(0.29633333333333333334e-1_f64) * t81179 - F::cast_from(0.9877777777777777778e-2_f64) * t81184 - F::cast_from(0.29633333333333333334e-1_f64) * t81188 + F::cast_from(0.26670000000000000001e0_f64) * t81192 + F::cast_from(0.35560000000000000001e0_f64) * t81196 + F::cast_from(0.88900000000000000002e-1_f64) * t81200 + F::cast_from(0.88900000000000000002e-1_f64) * t81204 + F::cast_from(0.29633333333333333334e-1_f64) * t81209 - F::cast_from(0.43901234567901234568e-1_f64) * t81214 - F::cast_from(0.16462962962962962963e-1_f64) * t68262 - F::cast_from(0.29633333333333333334e-1_f64) * t68277;
    t84156
}
