//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 825/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk825<F: Float>(t12658: F, t12659: F, t211: F, t3138: F, t1001: F, t213: F, t3174: F, t1050: F, t3253: F, t1042: F, t3271: F, t3137: F, t974: F) -> (F, F, F, F, F) {
    let t12660 = t12658 * t12659;
    let t12662 = t3138 * t211;
    let t12663 = t213 * t1001;
    let t12664 = t12663 * t3174;
    let t12665 = t12662 * t12664;
    let t12667 = t3253 * t1050;
    let t12669 = t1042 * t3271;
    let t12671 = t974 * t3137;
    (t12660, t12665, t12667, t12669, t12671)
}
