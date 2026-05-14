//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1132/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1132<F: Float>(t20526: F, t538: F, t6155: F, t1234: F, t1606: F, t6243: F, t2155: F, t1603: F, t2116: F, t4888: F, t5: F, t511: F, t7: F, t512: F, t57: F, t6101: F) -> (F, F, F, F, F) {
    let t20528 = t6155 * t538 * t20526;
    let t20530 = t1606 * t1234;
    let t20531 = t6243 * t20530;
    let t20532 = t2155 * t20531;
    let t20539 = 0.82757551241431752271e-2 * t5 * t7 * t4888 * t511 * t1603 * t2116;
    let t20541 = t512 * t6101 * t57;
    (t20528, t20531, t20532, t20539, t20541)
}
