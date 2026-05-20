//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3167/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3167<F: Float>(t43888: F, t56176: F, t56236: F, t56447: F, t56462: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> F {
    let t83230 = F::cast_from(0.55555555555555555556e-2_f64) * t68332 + F::cast_from(0.11111111111111111111e-1_f64) * t68334 + F::cast_from(0.33333333333333333333e-1_f64) * t68336 - F::cast_from(0.74074074074074074073e-2_f64) * t56176 + t56447 + F::new(0.15e0) * t81224 + F::cast_from(0.83333333333333333333e-2_f64) * t81228 - F::cast_from(0.30864197530864197531e-2_f64) * t81230 + F::cast_from(0.11111111111111111111e-1_f64) * t81232 - F::cast_from(0.16666666666666666667e-1_f64) * t81234 - F::cast_from(0.27777777777777777778e-2_f64) * t81236 + t56462 - F::cast_from(0.25925925925925925926e-1_f64) * t56236 - F::cast_from(0.83333333333333333334e-2_f64) * t68389 + F::cast_from(0.22222222222222222223e-1_f64) * t68399 + F::cast_from(0.27777777777777777777e-1_f64) * t81242 - F::cast_from(0.99999999999999999998e-1_f64) * t81245 - F::cast_from(0.86419753086419753087e-2_f64) * t43888 - F::cast_from(0.33333333333333333334e-1_f64) * t68454 - F::cast_from(0.50000000000000000001e-1_f64) * t68456;
    t83230
}
