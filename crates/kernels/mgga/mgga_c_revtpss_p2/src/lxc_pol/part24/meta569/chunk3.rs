//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1747/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1747<F: Float>(t43995: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F) -> F {
    let t90317 = -F::cast_from(0.13734567901234567901e-1_f64) * t81230 + F::cast_from(0.49444444444444444444e-1_f64) * t81232 - F::cast_from(0.16481481481481481482e-1_f64) * t68257 - F::cast_from(0.74166666666666666668e-1_f64) * t81234 - F::cast_from(0.12361111111111111111e-1_f64) * t81236 + F::cast_from(0.12361111111111111111e0_f64) * t89865 - F::cast_from(0.22249999999999999999e0_f64) * t89869 + F::cast_from(0.2225e0_f64) * t89873 + F::cast_from(0.92708333333333333333e-2_f64) * t89877 - F::cast_from(0.38456790123456790123e-1_f64) * t56236 + t43995 + F::cast_from(0.49444444444444444445e-1_f64) * t68399;
    t90317
}
