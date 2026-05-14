//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1096/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1096<F: Float>(t11388: F, t3065: F, t11479: F, t1912: F, t5285: F, t11326: F, t8885: F, t1882: F, t20461: F, t21249: F, t5462: F, t674: F, t11463: F, t505: F, t5713: F, t9066: F) -> (F, F, F, F, F) {
    let t35157 = t11388 * t3065;
    let t35160 = t5285 * t11479 * t1912;
    let t35162 = t11326 * t8885;
    let t35169 = t5462 * t1882 * t20461 * t674 * t21249;
    let t35173 = t11463 * t9066 * t505 * t5713;
    (t35157, t35160, t35162, t35169, t35173)
}
