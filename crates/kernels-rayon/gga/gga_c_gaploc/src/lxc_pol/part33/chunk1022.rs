//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1022/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1022(t12156: f64, t12191: f64, t12199: f64, t12202: f64, t12235: f64, t12247: f64, t12249: f64, t12267: f64, t3749: f64, t841: f64, t10287: f64, t10291: f64, t10292: f64, t10294: f64, t10303: f64, t10797: f64, t10798: f64, t10804: f64, t10807: f64, t12031: f64, t12034: f64, t12037: f64, t1955: f64, t1960: f64, t748: f64) -> (f64, f64, f64) {
    let t12270 = t12156 + t12191 + t12199 + t12202 + t12235 + t12247 + t12249 + t12267;
    let t12272 = t3749 * t841;
    let t12276 = -t12270 * t748 + 2.0_f64 * t12272 * t1960 - t1955 * t3749 - t10287 - t10291 + t10292 - t10294 - t10303 - t10797 - t10798 - t10804 + t10807 - t12031 + t12034 - t12037;
    (t12270, t12272, t12276)
}
