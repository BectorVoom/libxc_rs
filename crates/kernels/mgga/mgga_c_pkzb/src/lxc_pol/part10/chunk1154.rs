//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1154/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1154<F: Float>(t1505: F, t1618: F, t555: F, t127: F, t1541: F, t78: F, t5142: F, t546: F, t1670: F, t5322: F, t81: F, t79: F, t1613: F, t16588: F, t541: F, t146: F, t1540: F, t155: F, t52: F, t95: F) -> (F, F, F, F, F, F, F, F) {
    let t16923 = 0.21053605041484726346e2 * t555 * t1505 * t1618;
    let t16926 = t78 * t1541 * t127;
    let t16933 = t5142 * t546;
    let t16935 = t1670 * t5322;
    let t16942 = t81 * t81;
    let t16946 = 840.0 * t79 / t16942 * t127;
    let t16950 = 0.35089341735807877242e1 * t555 * t1613 * t16588 * t541;
    let t17026 = 455.0 / 243.0 * t146 / t52 / t1540 * t95 * t155;
    (t16923, t16926, t16933, t16935, t16942, t16946, t16950, t17026)
}
