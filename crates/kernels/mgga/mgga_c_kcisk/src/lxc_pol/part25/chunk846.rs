//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 846/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk846<F: Float>(t12663: F, t3174: F, t12662: F, t1050: F, t3253: F, t1042: F, t3271: F, t3137: F, t974: F, t3260: F, t3232: F, t981: F, t1036: F, t1039: F, t3139: F, t3241: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12664 = t12663 * t3174;
    let t12665 = t12662 * t12664;
    let t12667 = t3253 * t1050;
    let t12669 = t1042 * t3271;
    let t12671 = t974 * t3137;
    let t12672 = t12671 * t3260;
    let t12674 = t3232 * t981;
    let t12675 = t12674 * t1036;
    let t12677 = t1039 * t3139;
    let t12678 = t3241 * t12677;
    (t12664, t12665, t12667, t12669, t12671, t12672, t12674, t12675, t12678)
}
