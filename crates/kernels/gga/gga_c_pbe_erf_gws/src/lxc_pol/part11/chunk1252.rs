//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1252/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1252<F: Float>(t13544: F, t20607: F, t2277: F, t3257: F, t36814: F, t3836: F, t45017: F, t45620: F, t45703: F, t49087: F, t49845: F, t49852: F, t49857: F, t49859: F, t49861: F, t49875: F, t6384: F, t904: F, t929: F) -> F {
    let t49879 = -t49845 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t45620 + t49852 - t49857 - t49859 - t49861 + F::cast_from(11.0_f64) / F::cast_from(768.0_f64) * t2277 * t3257 * t36814 * t45017 - F::cast_from(15.0_f64) / F::cast_from(64.0_f64) * t929 * t6384 * t904 * t49087 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t45703 + t49875 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t20607 * t3836 * t13544;
    t49879
}
