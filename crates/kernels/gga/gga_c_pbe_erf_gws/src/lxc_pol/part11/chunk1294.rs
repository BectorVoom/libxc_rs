//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1294/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1294<F: Float>(t3931: F, t18850: F, t18853: F, t18863: F, t18920: F, t18924: F, t18933: F, t19517: F, t19525: F, t21885: F, t2429: F, t321: F, t382: F, t48497: F, t48498: F, t48499: F, t49423: F, t49425: F, t49426: F, t49955: F) -> F {
    let t50755 = t3931 * t3931;
    let t50759 = -F::cast_from(6.0_f64) * t21885 * t321 * t50755 + F::cast_from(18.0_f64) * t2429 * t382 * t49955 + t18850 + t18853 - t18863 + t18920 + t18924 - t18933 - t19517 + t19525 - t48497 - t48498 + t48499 - t49423 - t49425 - t49426;
    t50759
}
