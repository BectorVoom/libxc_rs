//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3110/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3110<F: Float>(t43771: F, t43814: F, t43817: F, t68255: F, t68257: F, t81156: F, t81158: F, t81162: F, t81167: F, t81399: F, t81401: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t81416: F) -> (F, F) {
    let t81904 = F::cast_from(0.40256666666666666668e0_f64) * t68255 - F::cast_from(0.26837777777777777777e0_f64) * t68257 - F::cast_from(0.24528888888888888889e0_f64) * t43771 + F::cast_from(0.20128333333333333333e0_f64) * t81156 - F::new(0.60385e0) * t81158 + F::cast_from(0.10064166666666666667e1_f64) * t81162 + F::cast_from(0.40256666666666666666e1_f64) * t81167 + F::new(0.258925e1) * t81399 + t43814 + t43817 + F::new(0.16504875e0) * t81401;
    let t81917 = -F::new(0.36231e1) * t81171 - F::new(0.72462e1) * t81175 - F::cast_from(0.60384999999999999999e0_f64) * t81179 - F::cast_from(0.20128333333333333333e0_f64) * t81184 - F::cast_from(0.60384999999999999999e0_f64) * t81188 + F::new(0.543465e1) * t81192 + F::new(0.72462e1) * t81196 + F::new(0.181155e1) * t81200 + F::new(0.181155e1) * t81204 + F::new(0.60385e0) * t81209 - F::cast_from(0.89459259259259259259e0_f64) * t81214 + F::new(0.11038e0) * t81416;
    (t81904, t81917)
}
