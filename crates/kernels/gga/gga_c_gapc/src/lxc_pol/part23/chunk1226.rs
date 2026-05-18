//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1226/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1226<F: Float>(t1882: F, t20461: F, t21249: F, t5462: F, t674: F, t11463: F, t505: F, t5713: F, t9066: F, t116: F, t33257: F, t3698: F, t3702: F) -> (F, F, F, F) {
    let t35169 = t5462 * t1882 * t20461 * t674 * t21249;
    let t35173 = t11463 * t9066 * t505 * t5713;
    let t35175 = t116 * t33257;
    let t35177 = t35175 * t3698 * t3702;
    (t35169, t35173, t35175, t35177)
}
