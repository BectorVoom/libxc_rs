//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1136/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1136<F: Float>(t11523: F, t11540: F, t2333: F, t3060: F, t795: F, t10997: F, t3275: F, t3229: F, t3276: F, t792: F, t8601: F, t12414: F) -> (F, F, F, F, F) {
    let t42417 = t11523 * t11540 / F::new(2.0);
    let t42418 = t2333 * t3060;
    let t42419 = t42418 * t795;
    let t42422 = F::new(45.0) / F::new(64.0) * t3275 * t10997 * t42419;
    let t42423 = t2333 * t3229;
    let t42424 = t42423 * t795;
    let t42427 = F::new(5.0) / F::new(16.0) * t3275 * t3276 * t42424;
    let t42428 = t8601 * t792;
    let t42431 = F::new(5.0) / F::new(16.0) * t3275 * t3276 * t42428;
    let t42432 = t12414 * t792;
    (t42417, t42422, t42427, t42431, t42432)
}
