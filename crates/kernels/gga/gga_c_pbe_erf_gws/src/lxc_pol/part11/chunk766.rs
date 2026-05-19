//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 766/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk766<F: Float>(t12517: F, t203: F, t184: F, t221: F, t10293: F, t10301: F, t11191: F, t12436: F, t12438: F, t12442: F, t12446: F, t12448: F, t12450: F, t12454: F, t12488: F, t4910: F, t8405: F, t8408: F, t8414: F) -> (F, F, F, F, F, F) {
    let t12518 = t203 * t12517;
    let t12519 = t12518 * t184;
    let t12521 = F::new(2.0) / F::new(15.0) * t12519 * t221;
    let t12524 = F::new(4.0) / F::new(15.0) * t10293;
    let t12525 = F::new(16.0) / F::new(45.0) * t10301;
    let t12526 = F::cast_from(0.32463124087094530131e0_f64) * t11191 + t12436 - t12438 - t12442 - t12446 + t12448 + t12450 + t12454 + t4910 + F::new(4.0) * t8405 + t12488 + t12521 + F::cast_from(0.21642082724729686754e0_f64) * t8408 + F::cast_from(0.64926248174189060262e0_f64) * t8414 + t12524 - t12525;
    (t12518, t12519, t12521, t12524, t12525, t12526)
}
