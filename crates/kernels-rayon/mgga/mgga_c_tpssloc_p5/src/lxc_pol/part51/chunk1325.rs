//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1325/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1325(t119878: f64, t607: f64, t1410: f64, t645: f64, t6504: f64, t8308: f64, t641: f64, t31: f64, t7440: f64, t1433: f64, t33106: f64, t8513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119879 = t119878 * t607;
    let t119883 = t1410 * t645;
    let t119888 = t8308 * t1410 * t6504;
    let t119891 = t1410 * t641;
    let t119897 = t8308 * t7440 * t31 * t607;
    let t119901 = t1433 * t31 * t607;
    let t119913 = t8513 * t33106 * t6504;
    (t119879, t119883, t119888, t119891, t119897, t119901, t119913)
}
