//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1408/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1408<F: Float>(t5170: F, t26195: F, t26197: F, t12366: F, t17656: F, t1056: F, t8582: F, t5186: F, t2993: F, t44583: F, t5171: F, t17436: F, t34434: F) -> (F, F, F, F, F, F, F, F) {
    let t59157 = t5170 * t5170;
    let t59160 = F::new(0.24954977986735470917e5) * t26195 * t59157 * t26197;
    let t59162 = F::new(24.0) * t12366 * t17656;
    let t59165 = F::new(24.0) * t8582 * t59157 * t1056;
    let t59166 = t5186 * t5186;
    let t59169 = F::new(6.0) * t2993 * t59166 * t1056;
    let t59171 = F::new(12.0) * t44583 * t5171;
    let t59173 = F::new(0.38596378373162651572e3) * t34434 * t17436;
    (t59157, t59160, t59162, t59165, t59166, t59169, t59171, t59173)
}
