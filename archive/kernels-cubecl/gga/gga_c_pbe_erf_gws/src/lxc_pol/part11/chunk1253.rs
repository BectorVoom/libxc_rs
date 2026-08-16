//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1253/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1253<F: Float>(t13525: F, t37750: F, t38234: F, t1134: F, t2168: F, t3139: F, t44741: F, t2170: F, t44254: F, t49483: F, t8903: F, t3814: F) -> (F, F, F, F, F) {
    let t49894 = t37750 * t13525 / F::cast_from(12.0_f64);
    let t49895 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t38234;
    let t49899 = t2168 * t3139 * t44741 * t1134 / F::cast_from(24.0_f64);
    let t49903 = t8903 * t2170 * t44254 * t49483 / F::cast_from(2.0_f64);
    let t49907 = t2168 * t2170 * t44254 * t3814 / F::cast_from(12.0_f64);
    (t49894, t49895, t49899, t49903, t49907)
}
