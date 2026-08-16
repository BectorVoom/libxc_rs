//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1157/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1157(t213: f64, t81968: f64, t1894: f64, t236: f64, t9458: f64, t23034: f64, t6546: f64, t23037: f64, t131: f64, t845: f64, t1878: f64, t209: f64) -> (f64, f64, f64, f64) {
    let t81969 = t81968 * t213;
    let t81972 = t81969 * t1894 * t236 * t9458;
    let t81979 = t6546 * t23034;
    let t81980 = t81979 * t23037;
    let t81982 = t845 * t131;
    let t81984 = t1878 * t81982 * t209;
    (t81972, t81979, t81980, t81984)
}
