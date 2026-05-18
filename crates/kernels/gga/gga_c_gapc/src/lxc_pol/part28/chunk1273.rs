//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1273/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1273<F: Float>(t11326: F, t8885: F, t1882: F, t20461: F, t21249: F, t5462: F, t674: F, t11463: F, t505: F, t5713: F, t9066: F, t116: F, t33257: F) -> (F, F, F, F) {
    let t35162 = t11326 * t8885;
    let t35169 = t5462 * t1882 * t20461 * t674 * t21249;
    let t35173 = t11463 * t9066 * t505 * t5713;
    let t35175 = t116 * t33257;
    (t35162, t35169, t35173, t35175)
}
