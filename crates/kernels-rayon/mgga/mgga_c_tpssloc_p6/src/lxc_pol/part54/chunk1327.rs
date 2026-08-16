//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1327/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1327(t119878: f64, t607: f64, t1410: f64, t645: f64, t641: f64, t1433: f64, t31: f64, t32: f64, t26502: f64, t3701: f64, t26114: f64, t8327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119879 = t119878 * t607;
    let t119883 = t1410 * t645;
    let t119891 = t1410 * t641;
    let t119901 = t1433 * t31 * t607;
    let t119931 = t32 * t607;
    let t120016 = t3701 * t26502;
    let t120067 = 2.0_f64 * t26114 * t8327;
    (t119879, t119883, t119891, t119901, t119931, t120016, t120067)
}
