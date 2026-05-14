//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 937/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk937<F: Float>(t13442: F, t2142: F, t44254: F, t6241: F, t2121: F, t337: F, t11781: F, t3916: F, t12041: F, t38761: F, t13254: F, t6402: F, t38143: F, t9035: F, t13271: F, t13282: F, t6484: F) -> (F, F, F, F, F, F, F, F) {
    let t45283 = t13442 * t2142;
    let t45304 = t44254 * t6241;
    let t45306 = t2121 * t337 * t45304;
    let t45320 = t3916 * t11781;
    let t45323 = t12041 * t38761;
    let t45345 = t6402 * t13254;
    let t45351 = t9035 * t38143;
    let t45353 = t6402 * t13271;
    let t45381 = t6484 * t13282;
    (t45283, t45306, t45320, t45323, t45345, t45351, t45353, t45381)
}
