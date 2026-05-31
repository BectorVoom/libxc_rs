//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1004/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1004<F: Float>(t5445: F, t723: F, t17493: F, t17498: F, t17501: F, t17503: F, t17507: F, t17511: F, t17514: F, t17517: F, t17520: F, t17523: F, t5390: F, t5451: F) -> F {
    let t18296 = t5445 * t723;
    let t18300 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t18296 + F::cast_from(0.2e-20_f64) * t5451 * t5390 - t17493 - t17498 - t17501 + t17503 + t17507 + t17511 + t17514 + t17517 - t17520 + t17523;
    t18300
}
