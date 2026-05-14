//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1365/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1365<F: Float>(t19034: F, t3759: F, t9461: F, t1308: F, t2158: F, t3491: F, t2173: F, t35869: F, t3961: F, t6204: F, t1339: F, t32000: F, t6229: F, t20149: F, t33476: F, t9446: F) -> (F, F, F, F, F) {
    let t114072 = t3759 * t9461 * t19034;
    let t114075 = t3491 * t2158 * t1308;
    let t114082 = t6204 * t35869 * t2173 * t3961;
    let t114092 = t1339 * t32000 * t6229;
    let t114095 = t9446 * t20149 * t33476;
    (t114072, t114075, t114082, t114092, t114095)
}
