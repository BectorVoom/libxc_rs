//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1260/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1260<F: Float>(t343: F, t50018: F, t13403: F, t2170: F, t3138: F, t44254: F, t45882: F, t2210: F, t49178: F, t858: F, t884: F, t11630: F, t11773: F) -> (F, F, F, F, F) {
    let t50019 = t50018 * t343;
    let t50027 = t3138 * t2170 * t44254 * t13403 / F::cast_from(2.0_f64);
    let t50036 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t45882;
    let t50041 = t884 * t2210 * t858 * t49178 / F::cast_from(4.0_f64);
    let t50043 = t11773 * t11630 / F::cast_from(16.0_f64);
    (t50019, t50027, t50036, t50041, t50043)
}
