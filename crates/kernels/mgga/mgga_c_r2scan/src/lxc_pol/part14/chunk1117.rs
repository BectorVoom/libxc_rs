//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1117/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1117<F: Float>(t39030: F, t40630: F, t40631: F, t12197: F, t1561: F, t3275: F, t3277: F, t10630: F, t12056: F, t3262: F, t3352: F, t41202: F, t3469: F, t40358: F, t10610: F, t3465: F, t40285: F) -> (F, F, F, F, F, F) {
    let t42330 = 3.0 * t40630 * t39030 * t40631;
    let t42331 = t1561 * t12197;
    let t42334 = 5.0 / 8.0 * t3275 * t42331 * t3277;
    let t42339 = 3.0 / 4.0 * t3262 * t12056 * t10630;
    let t42344 = t3275 * t41202 * t3352 / 2.0;
    let t42346 = t40358 * t3469 / 4.0;
    let t42349 = 3.0 / 2.0 * t10610 * t3465 * t40285;
    (t42330, t42334, t42339, t42344, t42346, t42349)
}
