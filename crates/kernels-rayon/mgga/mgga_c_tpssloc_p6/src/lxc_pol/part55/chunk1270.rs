//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1270/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1270(t119862: f64, t119867: f64, t119874: f64, t119877: f64, t123091: f64, t123093: f64, t123095: f64, t123097: f64, t123112: f64, t123113: f64, t123115: f64, t1458: f64, t2314: f64, t32572: f64, t32609: f64, t32674: f64, t32676: f64, t32679: f64, t34203: f64, t4034: f64, t4077: f64, t652: f64) -> f64 {
    let t125121 = -2.0_f64 * t1458 * t32572 * t652 - 2.0_f64 * t2314 * t34203 - 2.0_f64 * t32609 * t4077 - 2.0_f64 * t34203 * t4034 - t119862 - t119867 - t119874 + t119877 - 4.0_f64 * t123091 - 4.0_f64 * t123093 - 4.0_f64 * t123095 - 4.0_f64 * t123097 + 2.0_f64 * t123112 - 4.0_f64 * t123113 - 4.0_f64 * t123115 - t32674 - t32676 - t32679;
    t125121
}
