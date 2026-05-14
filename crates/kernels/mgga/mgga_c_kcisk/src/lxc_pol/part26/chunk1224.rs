//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1224/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1224<F: Float>(t32376: F, t9523: F, t1299: F, t1414: F, t32338: F, t9515: F, t32388: F, t9529: F, t18681: F, t2737: F, t2739: F, t9528: F, t3805: F, t9475: F, t9466: F, t1413: F, t394: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109707 = t32376 * t9523;
    let t109717 = t1414 * t1299;
    let t109756 = t9515 * t32338;
    let t109793 = t9529 * t32388;
    let t109797 = 0.19290123456790123457e-2 * t2737 * t18681 * t2739;
    let t109803 = t32376 * t9528;
    let t109832 = t3805 * t9475;
    let t109846 = t3805 * t9466;
    let t109882 = t1413 * t394;
    (t109707, t109717, t109756, t109793, t109797, t109803, t109832, t109846, t109882)
}
