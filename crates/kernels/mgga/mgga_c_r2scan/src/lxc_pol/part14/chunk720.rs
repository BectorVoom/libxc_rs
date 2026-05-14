//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 720/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk720<F: Float>(t2080: F, t780: F, t1234: F, t1632: F, t551: F, t566: F, t110: F, t6189: F, t6188: F, t6072: F, t2168: F, t2183: F, t2191: F, t2236: F, t2252: F, t549: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6455 = t2080 * t780;
    let t6457 = t1632 * t1234;
    let t6458 = t551 * t6457;
    let t6459 = t566 * t6458;
    let t6461 = t6189 * t110;
    let t6462 = t6188 * t6461;
    let t6463 = t6462 * t6072;
    let t6465 = t2183 * t2168;
    let t6468 = t2236 * t2191;
    let t6470 = t1632 * t2252;
    let t6471 = t551 * t6470;
    let t6472 = t549 * t6471;
    (t6455, t6457, t6459, t6461, t6462, t6463, t6465, t6468, t6470, t6472)
}
