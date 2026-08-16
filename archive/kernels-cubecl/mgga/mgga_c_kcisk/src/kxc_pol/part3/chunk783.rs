//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 783/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk783<F: Float>(t10552: F, t1685: F, t1676: F, t4753: F, t1670: F, t4787: F, t10690: F, t591: F, t4790: F, t10681: F, t10696: F, t10699: F) -> (F, F, F, F, F, F, F, F) {
    let t12087 = t10552 * t1685;
    let t12090 = t4753 * t1676;
    let t12095 = t1670 * t4787;
    let t12098 = t591 * t10690;
    let t12099 = t10552 * t4790;
    let t12102 = t10681 * t1685;
    let t12105 = t591 * t10696;
    let t12106 = t10552 * t10699;
    (t12087, t12090, t12095, t12098, t12099, t12102, t12105, t12106)
}
