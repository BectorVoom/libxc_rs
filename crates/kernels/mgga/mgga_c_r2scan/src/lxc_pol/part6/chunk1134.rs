//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1134/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1134<F: Float>(t1550: F, t495: F, t538: F, t7623: F, t5073: F, t537: F, t113: F, t2115: F, t2155: F, t2228: F, t6068: F, t6156: F, t20373: F, t2147: F, t2148: F, t2168: F, t6527: F) -> (F, F, F, F, F, F, F, F) {
    let t20565 = t495 * t1550;
    let t20567 = t7623 * t538 * t20565;
    let t20570 = t537 * t5073;
    let t20571 = t20570 * t113;
    let t20572 = t2115 * t20571;
    let t20573 = t2155 * t20572;
    let t20575 = t2228 * t6068;
    let t20576 = t20575 * t6156;
    let t20578 = t20373 * t113;
    let t20580 = t2147 * t2148 * t20578;
    let t20582 = t6527 * t2168;
    (t20565, t20567, t20572, t20573, t20575, t20576, t20580, t20582)
}
