//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1049/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1049<F: Float>(t2282: F, t8286: F, t4170: F, t30947: F, t467: F, t492: F, t500: F, t30494: F, t6317: F, t6316: F, t31165: F, t4231: F) -> (F, F, F, F) {
    let t31207 = t2282 * t8286;
    let t31209 = F::cast_from(6.0_f64) * t4170 * t31207;
    let t31210 = t30947 * t467;
    let t31211 = t31210 * t492;
    let t31212 = t31211 * t500;
    let t31214 = t6317 * t30494;
    let t31215 = t6316 * t31214;
    let t31217 = t4231 * t31165;
    (t31209, t31212, t31215, t31217)
}
