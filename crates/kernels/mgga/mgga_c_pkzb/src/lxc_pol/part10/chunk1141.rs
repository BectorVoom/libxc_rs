//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1141/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1141<F: Float>(t4885: F, t496: F, t1545: F, t1626: F, t4803: F, t513: F, t5142: F, t1527: F, t1516: F, t491: F, t1599: F, t1601: F, t490: F, t4993: F, t1597: F, t1517: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16502 = t496 * t4885;
    let t16504 = t1545 * t1626;
    let t16506 = t4803 * t513;
    let t16508 = t5142 * t513;
    let t16510 = t1527 * t1527;
    let t16513 = 6.0 * t1516 * t16510 * t491;
    let t16517 = 0.64327917994770140268e2 * t1599 * t4993 * t1601 * t490;
    let t16518 = t1597 * t1597;
    let t16521 = t1517 * t1517;
    (t16502, t16504, t16506, t16508, t16510, t16513, t16517, t16518, t16521)
}
