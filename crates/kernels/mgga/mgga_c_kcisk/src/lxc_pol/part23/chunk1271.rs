//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1271/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1271<F: Float>(t2737: F, t32484: F, t4419: F, t32401: F, t32474: F, t32394: F, t9515: F, t32433: F, t32342: F, t32436: F, t3805: F, t9475: F, t1333: F, t32081: F, t32059: F, t9466: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109801 = t2737 * t4419 * t32484;
    let t109806 = t32474 * t32401;
    let t109817 = t9515 * t32394;
    let t109820 = t32433 * t32401;
    let t109828 = t32436 * t32342;
    let t109832 = t3805 * t9475;
    let t109836 = t1333 * t32081;
    let t109838 = t1333 * t32059;
    let t109846 = t3805 * t9466;
    (t109801, t109806, t109817, t109820, t109828, t109832, t109836, t109838, t109846)
}
