//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 513/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk513<F: Float>(t1780: F, t231: F, t3401: F, t3405: F, t3409: F, t3413: F, t3417: F, t3419: F, t3447: F, t3449: F, t3453: F, t3458: F, t3459: F) -> F {
    let t3462 = t3401 + t3405 - t3409 + t3413 - t3417 - t3419 - t3447 + t3449 + t3453 + t3458 - t1780 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3459 * t231;
    t3462
}
