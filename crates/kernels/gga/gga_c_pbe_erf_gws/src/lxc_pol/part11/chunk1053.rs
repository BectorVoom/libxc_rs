//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1053/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1053<F: Float>(t1114: F, t13140: F, t346: F, t13442: F, t2142: F, t44254: F, t6241: F, t2121: F, t337: F, t11781: F, t3916: F, t12041: F, t38761: F) -> (F, F, F, F, F) {
    let t45248 = t1114 * t13140 * t346;
    let t45283 = t13442 * t2142;
    let t45304 = t44254 * t6241;
    let t45306 = t2121 * t337 * t45304;
    let t45320 = t3916 * t11781;
    let t45323 = t12041 * t38761;
    (t45248, t45283, t45306, t45320, t45323)
}
