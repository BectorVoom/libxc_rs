//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 970/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk970<F: Float>(t10655: F, t10946: F, t10810: F, t3429: F, t3457: F, t10922: F, t10992: F, t158: F, t2312: F, t3446: F, t37428: F, t3428: F, t3430: F, t6836: F, t870: F, t10684: F) -> (F, F, F, F, F, F, F) {
    let t38336 = t10655 * t10946;
    let t38339 = t3429 * t10810 * t3457;
    let t38341 = t10922 * t10946;
    let t38346 = t3446 * t10992 * t158 * t37428 * t2312;
    let t38349 = t6836 * t3428 * t3430;
    let t38355 = t870 * t10810;
    let t38356 = t38355 * t10684;
    (t38336, t38339, t38341, t38346, t38349, t38355, t38356)
}
