//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1249/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1249<F: Float>(t8001: F, t910: F, t2148: F, t6165: F, t2526: F, t2562: F, t113: F, t8694: F, t538: F, t7623: F, t41: F, t457: F, t8590: F, t1509: F, t3034: F, t468: F, t8967: F) -> (F, F, F, F, F, F, F) {
    let t27996 = t8001 * t910;
    let t27998 = t6165 * t2148 * t27996;
    let t28000 = t2562 * t2526;
    let t28002 = t6165 * t2148 * t28000;
    let t28005 = t8694 * t113;
    let t28007 = t7623 * t538 * t28005;
    let t28020 = t41 * t8590 * t457;
    let t28026 = t41 * t3034 * t1509;
    let t28027 = t8967 * t468;
    (t27998, t28002, t28005, t28007, t28020, t28026, t28027)
}
