//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1150/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1150<F: Float>(t13984: F, t14657: F, t13875: F, t13884: F, t13886: F, t13895: F, t14624: F, t14629: F, t14634: F, t14640: F, t14643: F, t14649: F, t14652: F, t14655: F, t2408: F, t3066: F, t335: F) -> F {
    let t14658 = t14657 * t13984;
    let t14660 = -F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t13875 + t3066 * t14624 / F::cast_from(48.0_f64) + t2408 * t14629 / F::cast_from(48.0_f64) + t14634 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t14640 - t335 * t14643 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t13884 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t13886 - t14649 / F::cast_from(96.0_f64) - t2408 * t14652 / F::cast_from(24.0_f64) + t13895 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t14655 - t14658 / F::cast_from(96.0_f64);
    t14660
}
