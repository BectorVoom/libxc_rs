//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1063/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1063<F: Float>(t21328: F, t4203: F, t4229: F, t5885: F, t4232: F, t13329: F, t19917: F, t6369: F, t21306: F, t21308: F, t21310: F, t21312: F, t21316: F, t21319: F, t21322: F, t21326: F) -> (F, F, F, F, F) {
    let t21329 = t4203 * t21328;
    let t21331 = t5885 * t4229;
    let t21332 = t21331 * t4232;
    let t21334 = t13329 * t4229;
    let t21335 = t6369 * t19917;
    let t21336 = t21334 * t21335;
    let t21338 = -t21306 / 72.0 - t21308 / 16.0 - t21310 / 96.0 - 2.0 / 9.0 * t21312 + t21316 / 8.0 - t21319 / 36.0 - t21322 / 12.0 - t21326 / 16.0 - t21329 / 24.0 + t21332 / 96.0 + 3.0 / 128.0 * t21336;
    (t21329, t21332, t21335, t21336, t21338)
}
