//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1273/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1273<F: Float>(t32473: F, t9528: F, t32358: F, t9515: F, t32338: F, t9511: F, t1333: F, t32078: F, t32480: F, t9532: F, t32353: F, t123: F, t2734: F, t32369: F, t32342: F, t32354: F) -> (F, F, F, F, F, F, F, F) {
    let t109896 = t32473 * t9528;
    let t109899 = t9515 * t32358;
    let t109910 = t9511 * t32338;
    let t109919 = t1333 * t32078;
    let t109921 = t32480 * t9532;
    let t109925 = t9511 * t32353;
    let t109929 = t2734 * t32369 * t123;
    let t109932 = t32354 * t32342;
    (t109896, t109899, t109910, t109919, t109921, t109925, t109929, t109932)
}
