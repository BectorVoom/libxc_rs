//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1303/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1303<F: Float>(t26282: F, t8085: F, t780: F, t9085: F, t2196: F, t2892: F, t551: F, t6343: F, t25852: F, t29288: F, t25967: F, t2687: F, t8089: F, t25972: F, t2691: F, t5100: F, t9319: F) -> (F, F, F, F, F, F, F) {
    let t31162 = t26282 * t8085;
    let t31178 = t9085 * t780;
    let t31182 = t2196 * t551 * t6343 * t2892;
    let t31184 = t25852 * t29288;
    let t31187 = t25967 * t2687 * t8089;
    let t31190 = t25972 * t2691 * t8089;
    let t31206 = t5100 * t9319;
    (t31162, t31178, t31182, t31184, t31187, t31190, t31206)
}
