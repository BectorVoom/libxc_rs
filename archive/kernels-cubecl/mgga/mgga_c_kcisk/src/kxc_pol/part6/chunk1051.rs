//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1051/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1051<F: Float>(t31141: F, t6369: F, t6368: F, t31165: F, t4204: F, t6331: F, t31170: F, t4231: F, t4230: F, t4203: F, t21331: F, t8271: F) -> (F, F, F, F, F) {
    let t31231 = t6369 * t31141;
    let t31232 = t6368 * t31231;
    let t31234 = t4204 * t31165;
    let t31235 = t6331 * t31234;
    let t31237 = t4231 * t31170;
    let t31238 = t4230 * t31237;
    let t31240 = t4204 * t31170;
    let t31241 = t4203 * t31240;
    let t31243 = t21331 * t8271;
    (t31232, t31235, t31238, t31241, t31243)
}
