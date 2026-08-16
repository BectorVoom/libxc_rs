//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1312/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1312(t31: f64, t607: f64, t7440: f64, t8308: f64, t1433: f64, t33106: f64, t6504: f64, t8513: f64, t32: f64, t33114: f64, t645: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119897 = t8308 * t7440 * t31 * t607;
    let t119901 = t1433 * t31 * t607;
    let t119913 = t8513 * t33106 * t6504;
    let t119931 = t32 * t607;
    let t119938 = t8513 * t33114 * t645;
    let t119942 = t79 * t7440;
    (t119897, t119901, t119913, t119931, t119938, t119942)
}
