//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1271/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1271<F: Float>(t394: F, t6309: F, t4208: F, t487: F, t1486: F, t21289: F, t1299: F, t6387: F, t21314: F, t4169: F, t9827: F, t20160: F, t33345: F, t9446: F, t32189: F, t33451: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113375 = t6309 * t394;
    let t113378 = t4208 * t487;
    let t113421 = t1486 * t487;
    let t113430 = t21289 * t394;
    let t113478 = t6387 * t1299;
    let t113497 = t21314 * t394;
    let t113573 = t9827 * t4169;
    let t113576 = t20160 * t33345;
    let t113578 = 0.69444444444444444446e-2 * t9446 * t113576;
    let t113579 = t32189 * t33451;
    (t113375, t113378, t113421, t113430, t113478, t113497, t113573, t113576, t113578, t113579)
}
