//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1336/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1336(t81778: f64, t81845: f64, t81905: f64, t81974: f64, t23034: f64, t6546: f64, t23037: f64, t131: f64, t845: f64, t1878: f64, t209: f64, t6637: f64, t6638: f64, t9458: f64) -> (f64, f64, f64, f64, f64) {
    let t81976 = t81778 + t81845 + t81905 + t81974;
    let t81979 = t6546 * t23034;
    let t81980 = t81979 * t23037;
    let t81982 = t845 * t131;
    let t81984 = t1878 * t81982 * t209;
    let t81987 = t81984 * t6637 * t6638 * t9458;
    (t81976, t81979, t81980, t81984, t81987)
}
