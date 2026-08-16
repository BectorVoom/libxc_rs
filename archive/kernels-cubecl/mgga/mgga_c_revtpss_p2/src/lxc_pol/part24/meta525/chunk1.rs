//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1557/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1557<F: Float>(t21213: F, t5357: F, t1256: F, t24681: F, t24671: F, t21233: F, t5391: F, t1261: F, t24240: F, t247: F, t3634: F, t21192: F, t5381: F) -> (F, F, F, F, F, F) {
    let t83316 = t21213 * t5357;
    let t83369 = t24681 * t1256;
    let t83371 = t24671 * t1256;
    let t83382 = t5391 * t21233;
    let t83392 = t1261 * t247 * t3634 * t24240;
    let t83394 = t5381 * t21192;
    (t83316, t83369, t83371, t83382, t83392, t83394)
}
