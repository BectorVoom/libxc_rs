//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3667/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3667<F: Float>(t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68402: F, t68464: F) -> F {
    let t69279 = -F::cast_from(0.68863333333333333332e0_f64) * t56187 - F::cast_from(0.20659e1_f64) * t56189 + F::cast_from(0.45908888888888888888e0_f64) * t56209 + F::cast_from(0.22954444444444444444e0_f64) * t56212 + F::cast_from(0.13772666666666666666e1_f64) * t56214 - F::cast_from(0.38257407407407407407e0_f64) * t56216 + F::cast_from(0.91817777777777777776e0_f64) * t56228 - F::cast_from(0.34431666666666666666e0_f64) * t56230 - F::cast_from(0.10712074074074074074e1_f64) * t56236 - F::cast_from(0.34431666666666666666e0_f64) * t68389 + F::cast_from(0.516475e0_f64) * t68393 - F::cast_from(0.68863333333333333334e0_f64) * t68397 + F::cast_from(0.45908888888888888889e0_f64) * t68399 + F::cast_from(0.46308888888888888889e-1_f64) * t68402 + F::cast_from(0.3529725e1_f64) * t68464;
    t69279
}
