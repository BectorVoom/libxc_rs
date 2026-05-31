//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1295/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1295<F: Float>(t18885: F, t18939: F, t18941: F, t18950: F, t18954: F, t18956: F, t18959: F, t19537: F, t2429: F, t321: F, t3703: F, t3931: F, t3932: F, t39758: F, t48502: F, t48503: F, t48504: F, t48506: F, t48507: F, t48508: F) -> F {
    let t50767 = -F::cast_from(36.0_f64) * t2429 * t3703 * t3932 + F::cast_from(12.0_f64) * t321 * t3931 * t39758 + t18885 + t18939 + t18941 - t18950 + t18954 + t18956 + t18959 - t19537 + t48502 + t48503 - t48504 + t48506 + t48507 - t48508;
    t50767
}
