//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 749/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk749<F: Float>(t2328: F, t7217: F, t2326: F, t190: F, t864: F, t136: F, t3: F, t362: F, t770: F, t2329: F, t850: F, t2336: F, t857: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7218 = t2328 * t7217;
    let t7219 = t2326 * t7218;
    let t7220 = t864 * t190;
    let t7221 = t7220 * t136;
    let t7222 = t3 * t362;
    let t7224 = t7221 * t7222 * t770;
    let t7227 = t2329 * t850;
    let t7228 = F::cast_from(1.0_f64) / t7227;
    let t7229 = t2328 * t7228;
    let t7230 = t2326 * t7229;
    let t7234 = t857 * t190 * t2336;
    (t7218, t7219, t7221, t7222, t7224, t7228, t7229, t7230, t7234)
}
