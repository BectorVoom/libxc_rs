//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1392/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1392<F: Float>(t52715: F, t55633: F, t55634: F, t57213: F, t57216: F, t57219: F, t57223: F, t57225: F, t57227: F, t57229: F, t57231: F, t57233: F, t57235: F) -> F {
    let t58776 = -t52715 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57213 + t55633 - t55634 + t57216 / F::cast_from(48.0_f64) - t57219 / F::cast_from(24.0_f64) - t57223 / F::cast_from(48.0_f64) + t57225 / F::cast_from(32.0_f64) + t57227 / F::cast_from(192.0_f64) + t57229 / F::cast_from(24.0_f64) - t57231 / F::cast_from(192.0_f64) + t57233 / F::cast_from(24.0_f64) + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t57235;
    t58776
}
