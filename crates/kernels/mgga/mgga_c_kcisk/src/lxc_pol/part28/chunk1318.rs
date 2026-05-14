//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1318/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1318<F: Float>(t1872: F, t2454: F, t34086: F, t4811: F, t32995: F, t34122: F, t10473: F, t9946: F, t34125: F, t32909: F, t34225: F, t5014: F, t9650: F, t62249: F, t9664: F, t9939: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116145 = t1872 * t2454;
    let t116149 = t4811 * t34086;
    let t116150 = 0.22109259259259259258e-2 * t116149;
    let t116156 = 0.69444444444444444446e-2 * t34122 * t32995;
    let t116167 = t10473 * t9946;
    let t116170 = 0.18518518518518518519e-1 * t34125 * t32995;
    let t116176 = t34225 * t32909;
    let t116186 = t5014 * t9650;
    let t116197 = t9664 * t62249 * t9939;
    (t116145, t116149, t116150, t116156, t116167, t116170, t116176, t116186, t116197)
}
