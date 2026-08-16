//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1216/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1216(t31011: f64, t3966: f64, t8513: f64, t32: f64, t607: f64, t2240: f64, t1409: f64, t8308: f64, t33118: f64, t645: f64, t46104: f64, t8301: f64) -> (f64, f64, f64, f64, f64) {
    let t119928 = t8513 * t31011 * t3966;
    let t119931 = t32 * t607;
    let t119932 = t2240 * t119931;
    let t119933 = t8308 * t1409;
    let t119948 = t8513 * t33118 * t645;
    let t119955 = t46104 * t8301;
    (t119928, t119932, t119933, t119948, t119955)
}
