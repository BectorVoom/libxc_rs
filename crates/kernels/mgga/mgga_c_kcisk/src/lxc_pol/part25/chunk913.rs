//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 913/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk913<F: Float>(t1060: F, t6791: F, t2488: F, t3293: F, t5101: F, t6790: F, t1824: F, t4684: F, t6746: F, t16022: F, t1835: F, t16009: F, t706: F, t1842: F, t1659: F, t1856: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16125 = t6791 * t1060;
    let t16128 = t2488 * t3293;
    let t16131 = t5101 * t6790;
    let t16132 = t16131 * t1824;
    let t16135 = t6746 * t4684;
    let t16138 = t1835 * t16022;
    let t16141 = t706 * t16009;
    let t16144 = t1842 * t16022;
    let t16147 = t1659 * t16009;
    let t16150 = t1856 * t16022;
    (t16125, t16128, t16132, t16135, t16138, t16141, t16144, t16147, t16150)
}
