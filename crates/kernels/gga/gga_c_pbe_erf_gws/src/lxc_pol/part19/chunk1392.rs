//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1392/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1392<F: Float>(t52715: F, t55633: F, t55634: F, t57213: F, t57216: F, t57219: F, t57223: F, t57225: F, t57227: F, t57229: F, t57231: F, t57233: F, t57235: F) -> F {
    let t58776 = -t52715 + F::new(7.0) / F::new(288.0) * t57213 + t55633 - t55634 + t57216 / F::new(48.0) - t57219 / F::new(24.0) - t57223 / F::new(48.0) + t57225 / F::new(32.0) + t57227 / F::new(192.0) + t57229 / F::new(24.0) - t57231 / F::new(192.0) + t57233 / F::new(24.0) + F::new(5.0) / F::new(96.0) * t57235;
    t58776
}
