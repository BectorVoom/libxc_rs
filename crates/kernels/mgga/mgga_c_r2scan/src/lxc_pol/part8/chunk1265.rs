//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1265/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1265<F: Float>(t1592: F, t1632: F, t551: F, t9110: F, t3056: F, t6212: F, t6209: F, t6211: F, t5100: F, t9423: F, t2892: F, t494: F, t6118: F, t8742: F, t560: F, t8832: F) -> (F, F, F, F, F, F) {
    let t29185 = t1592 * t551 * t1632 * t9110;
    let t29194 = t6212 * t3056;
    let t29196 = t6209 * t6211 * t29194;
    let t29207 = t5100 * t9423;
    let t29222 = t2892 * t494;
    let t29249 = t6118 * t8742;
    let t29270 = t8832 * t560;
    (t29185, t29196, t29207, t29222, t29249, t29270)
}
