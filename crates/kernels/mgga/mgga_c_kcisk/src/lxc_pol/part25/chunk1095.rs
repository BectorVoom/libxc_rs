//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1095/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1095<F: Float>(t32909: F, t9664: F, t5038: F, t9670: F, t7261: F, t4644: F, t9665: F, t1775: F, t1763: F, t4823: F, t1772: F) -> (F, F, F, F, F, F, F) {
    let t32910 = t9664 * t32909;
    let t32912 = t9670 * t5038;
    let t32913 = t7261 * t32912;
    let t32916 = t9665 * t4644;
    let t32917 = t1775 * t32916;
    let t32920 = t4823 * t1763;
    let t32921 = t32920 * t1772;
    (t32910, t32912, t32913, t32916, t32917, t32920, t32921)
}
