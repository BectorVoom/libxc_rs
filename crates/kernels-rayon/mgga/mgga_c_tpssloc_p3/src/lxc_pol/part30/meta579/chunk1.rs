//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1956/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1956(t1442: f64, t1774: f64, t1849: f64, t1869: f64, t1976: f64, t28819: f64, t28822: f64, t28825: f64, t28829: f64, t28833: f64, t28837: f64, t28841: f64, t28843: f64, t28852: f64, t28855: f64, t28861: f64, t28863: f64, t28866: f64, t4028: f64, t5450: f64, t5457: f64, t6287: f64, t652: f64, t7451: f64, t7472: f64, t7670: f64, t7681: f64) -> f64 {
    let t28867 = -2.0_f64 * t1442 * t7670 - 2.0_f64 * t1774 * t7451 + 2.0_f64 * t1849 * t7681 - t1869 * t6287 - t1976 * t5450 - 2.0_f64 * t1976 * t5457 - 2.0_f64 * t28852 * t652 - 4.0_f64 * t28855 * t652 - 4.0_f64 * t4028 * t7472 + t28819 + t28822 + t28825 + t28829 - t28833 + t28837 + t28841 + t28843 - t28861 - t28863 - t28866;
    t28867
}
