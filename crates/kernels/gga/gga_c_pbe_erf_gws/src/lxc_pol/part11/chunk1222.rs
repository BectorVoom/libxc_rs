//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1222/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1222<F: Float>(t36699: F, t18424: F, t18428: F, t18432: F, t18435: F, t18439: F, t18445: F, t18452: F, t18456: F, t18460: F, t18467: F, t18471: F, t18474: F, t48935: F) -> (F, F) {
    let t49415 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t36699;
    let t49416 = t18424 - t18428 + t18432 - t18435 + t18439 - t18445 - t18452 + t18456 - t18460 + t18467 - t18471 - t18474 - t48935;
    (t49415, t49416)
}
