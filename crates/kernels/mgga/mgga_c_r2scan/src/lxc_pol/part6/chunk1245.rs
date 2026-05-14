//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1245/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1245<F: Float>(t23232: F, t23235: F, t23236: F, t23237: F, t23238: F, t23240: F, t4987: F, t5009: F, t6777: F, t6780: F, t7009: F, t7011: F, t7018: F, t5021: F, t7035: F, t23241: F, t23244: F, t23245: F, t23249: F, t5012: F, t5015: F, t6783: F, t7038: F, t9911: F, t9912: F) -> (F, F) {
    let t23291 = -0.15584273195113317383e3 * t4987 - t23232 + t6777 + t6780 + 3.0 * t7009 + 9.0 * t7011 + t23235 - t23236 - t23237 - t23238 - 0.35089341735807877242e1 * t5009 + 36.0 * t7018 - t23240;
    let t23296 = 24.0 * t5021;
    let t23297 = 0.10986868383603927032e-2 * t7035;
    let t23299 = t23241 - 0.10986868383603927032e-2 * t5012 + 0.21973736767207854065e-2 * t5015 - t23244 + t23245 + t6783 + t23296 + t9911 + t9912 - t23297 - t23249 + 9.0 * t7038;
    (t23291, t23299)
}
