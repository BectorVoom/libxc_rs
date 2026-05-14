//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 936/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk936<F: Float>(t13220: F, t6: F, t254: F, t1105: F, t2407: F, t3835: F, t858: F, t11564: F, t8833: F, t326: F, t38036: F, t13292: F, t6183: F, t9119: F, t1114: F, t13140: F, t346: F) -> (F, F, F, F, F, F, F) {
    let t45200 = t6 * t13220;
    let t45201 = t254 * t45200;
    let t45209 = t2407 * t858 * t3835 * t1105;
    let t45228 = t11564 * t8833;
    let t45235 = t326 * t38036;
    let t45240 = t9119 * t6183 * t13292;
    let t45248 = t1114 * t13140 * t346;
    (t45200, t45201, t45209, t45228, t45235, t45240, t45248)
}
