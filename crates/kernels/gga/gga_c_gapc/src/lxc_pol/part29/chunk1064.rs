//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1064/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1064<F: Float>(t11463: F, t505: F, t5713: F, t9066: F, t116: F, t33257: F, t3698: F, t3702: F, t198: F, t22118: F, t34195: F, t34197: F, t11347: F, t3091: F, t3670: F, t9356: F) -> (F, F, F, F, F, F) {
    let t35173 = t11463 * t9066 * t505 * t5713;
    let t35175 = t116 * t33257;
    let t35177 = t35175 * t3698 * t3702;
    let t35182 = t116 * t34195 * t34197 * t198 * t22118;
    let t35184 = t11347 * t3091;
    let t35186 = t3670 * t9356;
    (t35173, t35175, t35177, t35182, t35184, t35186)
}
