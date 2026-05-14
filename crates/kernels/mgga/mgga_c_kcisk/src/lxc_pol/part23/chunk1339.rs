//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1339/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1339<F: Float>(t21047: F, t9491: F, t109229: F, t9836: F, t1513: F, t19861: F, t21328: F, t21314: F, t394: F, t32261: F, t21300: F, t32260: F, t109302: F, t9839: F, t3508: F, t6340: F) -> (F, F, F, F, F, F, F, F) {
    let t113489 = t9491 * t21047;
    let t113491 = t109229 * t9836;
    let t113493 = t19861 * t1513;
    let t113495 = t9491 * t21328;
    let t113497 = t21314 * t394;
    let t113498 = t113497 * t32261;
    let t113500 = t32260 * t21300;
    let t113502 = t109302 * t9839;
    let t113504 = t3508 * t6340;
    (t113489, t113491, t113493, t113495, t113498, t113500, t113502, t113504)
}
