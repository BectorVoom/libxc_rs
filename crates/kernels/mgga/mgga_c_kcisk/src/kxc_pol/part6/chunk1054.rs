//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1054/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1054<F: Float>(t2267: F, t8233: F, t2259: F, t8252: F, t30226: F, t470: F, t487: F, t14365: F, t2271: F, t8283: F, t499: F, t498: F) -> (F, F, F, F, F) {
    let t31261 = t8233 * t2267;
    let t31263 = t2259 * t8252;
    let t31265 = t470 * t30226;
    let t31266 = t487 * t31265;
    let t31267 = t14365 * t31266;
    let t31269 = t2271 * t8283;
    let t31271 = t499 * t30226;
    let t31272 = t498 * t31271;
    (t31261, t31263, t31267, t31269, t31272)
}
