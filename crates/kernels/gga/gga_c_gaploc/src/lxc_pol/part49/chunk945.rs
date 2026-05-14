//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 945/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk945<F: Float>(t2679: F, t3726: F, t9796: F, t12240: F, t2617: F, t7810: F, t38961: F, t935: F, t1457: F, t2103: F, t43444: F, t43447: F, t43448: F, t43449: F, t43450: F, t43455: F, t43456: F, t43458: F, t43462: F) -> (F, F) {
    let t47212 = t9796 * t3726 * t2679;
    let t47215 = t7810 * t12240 * t2617;
    let t47220 = t38961 * t935;
    let t47222 = t2103 * t1457 * t47220;
    let t47224 = -0.38342925953920749676e0 * t47212 - 0.19171462976960374838e0 * t47215 - t43444 - t43447 + t43448 - t43449 + t43450 - t43455 + 0.35750489951850426669e0 * t43456 + 0.14896037479937677779e-1 * t43458 + 0.14896037479937677779e-1 * t43462 + 0.71500979903700853338e0 * t47222;
    (t47220, t47224)
}
