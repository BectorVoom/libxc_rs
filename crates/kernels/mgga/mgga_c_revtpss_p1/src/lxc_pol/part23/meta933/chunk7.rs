//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3070/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3070<F: Float>(t43888: F, t56176: F, t56184: F, t56229: F, t56236: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> F {
    let t81250 = F::cast_from(0.61805555555555555556e-2_f64) * t68332 + F::cast_from(0.12361111111111111111e-1_f64) * t68334 + F::cast_from(0.37083333333333333333e-1_f64) * t68336 - F::cast_from(0.82407407407407407407e-2_f64) * t56176 + t56184 + F::cast_from(0.166875e0_f64) * t81224 + F::cast_from(0.92708333333333333333e-2_f64) * t81228 - F::cast_from(0.34336419753086419753e-2_f64) * t81230 + F::cast_from(0.12361111111111111111e-1_f64) * t81232 - F::cast_from(0.18541666666666666667e-1_f64) * t81234 - F::cast_from(0.30902777777777777778e-2_f64) * t81236 + t56229 - F::cast_from(0.28842592592592592592e-1_f64) * t56236 - F::cast_from(0.92708333333333333334e-2_f64) * t68389 + F::cast_from(0.24722222222222222223e-1_f64) * t68399 + F::cast_from(0.30902777777777777777e-1_f64) * t81242 - F::cast_from(0.11125e0_f64) * t81245 - F::cast_from(0.96141975308641975307e-2_f64) * t43888 - F::cast_from(0.37083333333333333334e-1_f64) * t68454 - F::cast_from(0.55625000000000000001e-1_f64) * t68456;
    t81250
}
