//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 656/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk656<F: Float>(t3354: F, t478: F, t3629: F, t3631: F, t3633: F) -> (F, F) {
    let t3635 = t478 * t3354;
    let t3637 = -t3629 / F::cast_from(9.0_f64) + t3631 / F::cast_from(3.0_f64) - t3633 / F::cast_from(9.0_f64) + t3635 / F::cast_from(3.0_f64);
    (t3635, t3637)
}
