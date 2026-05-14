//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1077/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1077<F: Float>(t39977: F, t39982: F, t41634: F, t41635: F, t41636: F, t41637: F, t41638: F, t41642: F, t41644: F, t43447: F, t43451: F, t43454: F, t38033: F, t41649: F, t41651: F, t43459: F, t43462: F, t43465: F, t43468: F, t43471: F, t43474: F, t43477: F, t43480: F, t43483: F) -> (F, F) {
    let t44396 = 0.27944763721877274748e0 * t43447 - 0.46574606203128791246e-1 * t43451 + t41634 + t41635 + 0.12805040077930161442e0 * t43454 + t41636 + t41637 + t41638 - 0.85366933852867742947e0 * t39977 - t41642 - 0.92461031893912198008e0 * t39982 + t41644;
    let t44407 = 0.17465477326173296718e-1 * t43459 + 0.26198215989259945076e-1 * t43462 - 0.87327386630866483588e-2 * t43465 + 0.26198215989259945076e-1 * t43468 + 0.1047928639570397803e0 * t43471 + t41649 + t41651 + 0.86682217400542685632e-1 * t43474 - 0.87327386630866483588e-2 * t43477 + 0.31147743054556651237e-1 * t38033 - 0.87327386630866483588e-2 * t43480 - 0.43663693315433241794e-2 * t43483;
    (t44396, t44407)
}
