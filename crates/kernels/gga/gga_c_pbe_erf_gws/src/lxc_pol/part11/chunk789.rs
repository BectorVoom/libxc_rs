//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 789/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk789<F: Float>(t11912: F, t11922: F, t12054: F, t3180: F, t3772: F, t5: F, t337: F, t2121: F, t3116: F, t13347: F, t2170: F, t3131: F, t2168: F, t13220: F, t6384: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13485 = 7.0 / 96.0 * t11912;
    let t13486 = 7.0 / 16.0 * t11922;
    let t13488 = t12054 * t3180 / 16.0;
    let t13489 = t5 * t3772;
    let t13490 = t337 * t13489;
    let t13491 = t2121 * t13490;
    let t13493 = t3116 * t13491 / 96.0;
    let t13496 = t2170 * t3131 * t13347;
    let t13498 = t2168 * t13496 / 16.0;
    let t13500 = t6384 * t904 * t13220;
    (t13485, t13486, t13488, t13489, t13490, t13491, t13493, t13496, t13498, t13500)
}
