//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1001/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1001<F: Float>(t11505: F, t494: F, t97: F, t3446: F, t37475: F, t970: F, t105: F, t2530: F, t113: F, t3578: F, t11056: F, t2378: F, t2381: F, t37028: F, t37078: F, t1010: F, t1276: F) -> (F, F, F, F, F, F, F, F) {
    let t40664 = t97 * t11505 * t494;
    let t40672 = t3446 * t37475 * t970;
    let t40681 = t97 * t105 * t2530;
    let t40713 = t97 * t3578 * t113;
    let t40779 = t2378 * t11056;
    let t40781 = t37028 * t2381;
    let t40786 = 44.0 / 9.0 * t37078;
    let t40788 = t1276 * t11056 * t1010;
    (t40664, t40672, t40681, t40713, t40779, t40781, t40786, t40788)
}
