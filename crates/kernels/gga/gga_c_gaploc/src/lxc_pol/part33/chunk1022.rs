//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1022/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1022<F: Float>(t12156: F, t12191: F, t12199: F, t12202: F, t12235: F, t12247: F, t12249: F, t12267: F, t3749: F, t841: F, t10287: F, t10291: F, t10292: F, t10294: F, t10303: F, t10797: F, t10798: F, t10804: F, t10807: F, t12031: F, t12034: F, t12037: F, t1955: F, t1960: F, t748: F) -> (F, F, F) {
    let t12270 = t12156 + t12191 + t12199 + t12202 + t12235 + t12247 + t12249 + t12267;
    let t12272 = t3749 * t841;
    let t12276 = -t12270 * t748 + F::cast_from(2.0_f64) * t12272 * t1960 - t1955 * t3749 - t10287 - t10291 + t10292 - t10294 - t10303 - t10797 - t10798 - t10804 + t10807 - t12031 + t12034 - t12037;
    (t12270, t12272, t12276)
}
