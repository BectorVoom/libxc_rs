//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1356/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1356<F: Float>(t2567: F, t3016: F, t2148: F, t7614: F, t10108: F, t24107: F, t24774: F, t24777: F, t24860: F, t29249: F, t29272: F, t29276: F, t29281: F, t29285: F, t29289: F, t6425: F, t8240: F, t9209: F, t9214: F, t9339: F, t9441: F) -> (F,) {
    let t33145 = t2567 * t3016;
    let t33147 = t7614 * t2148 * t33145;
    let t33152 = 0.7801399566048841707e0 * t8240 * t9209 + 0.7801399566048841707e0 * t8240 * t9214 + 0.39006997830244208535e0 * t6425 * t10108 - 0.78013995660488417067e0 * t24860 * t9339 - 0.78013995660488417067e0 * t24107 * t9441 + 0.24393601348456957547e-3 * t24774 - t24777 + 0.1536604809351619373e1 * t29249 + 0.52396431978519890151e-1 * t29272 + 0.2095857279140795606e0 * t29276 + 0.20958572791407956061e0 * t33147 - 0.1047928639570397803e0 * t29281 + 0.1047928639570397803e0 * t29285 + 0.65854491829355115984e-1 * t29289;
    (t33152,)
}
