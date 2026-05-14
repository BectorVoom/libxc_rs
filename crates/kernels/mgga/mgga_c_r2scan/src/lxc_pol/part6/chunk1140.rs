//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1140/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1140<F: Float>(t489: F, t524: F, t6238: F, t2225: F, t20473: F, t503: F, t19879: F, t538: F, t2110: F, t2162: F, t2304: F, t6162: F, t1568: F, t20242: F, t7623: F, t2168: F, t5135: F) -> (F, F, F, F, F, F, F) {
    let t20758 = t524 * t6238 * t489;
    let t20759 = t20758 * t2225;
    let t20762 = t503 * t20473;
    let t20764 = t20762 * t538 * t19879;
    let t20769 = 0.25059275625254849634e-3 * t2304 * t2110 * t2162 * t6162;
    let t20771 = t7623 * t1568 * t20242;
    let t20773 = t5135 * t2168;
    (t20758, t20759, t20762, t20764, t20769, t20771, t20773)
}
