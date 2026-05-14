//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 909/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk909<F: Float>(t551: F, t552: F, t6450: F, t2080: F, t780: F, t1234: F, t1632: F, t566: F, t110: F, t6189: F, t6188: F, t6072: F, t2168: F, t2183: F) -> (F, F, F, F, F, F, F) {
    let t6452 = t551 * t552 * t6450;
    let t6455 = t2080 * t780;
    let t6457 = t1632 * t1234;
    let t6458 = t551 * t6457;
    let t6459 = t566 * t6458;
    let t6461 = t6189 * t110;
    let t6462 = t6188 * t6461;
    let t6463 = t6462 * t6072;
    let t6465 = t2183 * t2168;
    (t6452, t6455, t6458, t6459, t6462, t6463, t6465)
}
