//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1214/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1214(t1983: f64, t32782: f64, t6999: f64, t24987: f64, t8490: f64, t1437: f64, t31: f64, t607: f64, t8308: f64, t1410: f64, t645: f64, t641: f64) -> (f64, f64, f64, f64, f64) {
    let t119874 = t1983 * t32782 * t6999;
    let t119877 = t24987 * t8490;
    let t119878 = t1437 * t31;
    let t119879 = t119878 * t607;
    let t119880 = t8308 * t119879;
    let t119883 = t1410 * t645;
    let t119884 = t8308 * t119883;
    let t119891 = t1410 * t641;
    (t119874, t119877, t119880, t119884, t119891)
}
