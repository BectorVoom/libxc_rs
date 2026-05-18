//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1146/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1146<F: Float>(t2333: F, t3060: F, t795: F, t3229: F, t792: F, t8601: F, t12414: F, t2892: F, t12574: F, t23495: F, t3629: F, t11888: F, t8358: F) -> (F, F, F, F, F, F, F, F) {
    let t42418 = t2333 * t3060;
    let t42419 = t42418 * t795;
    let t42423 = t2333 * t3229;
    let t42424 = t42423 * t795;
    let t42428 = t8601 * t792;
    let t42432 = t12414 * t792;
    let t42453 = t2333 * t2892;
    let t42454 = t42453 * t795;
    let t42472 = t12574 * t792;
    let t42491 = t23495 * t3629;
    let t42493 = t8358 * t11888;
    (t42419, t42424, t42428, t42432, t42454, t42472, t42491, t42493)
}
