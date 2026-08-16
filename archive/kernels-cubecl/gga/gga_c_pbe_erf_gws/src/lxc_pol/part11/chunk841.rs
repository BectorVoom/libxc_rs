//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 841/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk841<F: Float>(t13290: F, t5: F, t337: F, t2121: F, t9119: F, t1149: F, t12024: F, t11478: F, t2170: F, t3814: F, t2168: F, t3131: F, t3139: F, t3855: F) -> (F, F, F, F, F, F, F, F) {
    let t13291 = t5 * t13290;
    let t13292 = t337 * t13291;
    let t13293 = t2121 * t13292;
    let t13295 = t9119 * t13293 / F::cast_from(48.0_f64);
    let t13296 = t12024 * t1149;
    let t13300 = t2170 * t11478 * t3814;
    let t13302 = t2168 * t13300 / F::cast_from(16.0_f64);
    let t13304 = t3139 * t3131 * t3855;
    (t13291, t13292, t13293, t13295, t13296, t13300, t13302, t13304)
}
