//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 519/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk519<F: Float>(t199: F, t3488: F, t1022: F, t209: F, t184: F) -> (F, F, F, F) {
    let t3490 = F::new(2.0) / F::new(15.0) * t3488 * t199;
    let t3491 = t1022 * t1022;
    let t3492 = t3491 * t209;
    let t3493 = t3492 * t184;
    (t3490, t3491, t3492, t3493)
}
