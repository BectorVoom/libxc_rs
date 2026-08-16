//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1054/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1054(t31130: f64, t3887: f64, t1375: f64, t31094: f64, t31096: f64, t31103: f64, t31106: f64, t31111: f64, t31113: f64, t31115: f64, t31117: f64, t31122: f64, t31126: f64, t31129: f64, t6958: f64, t6993: f64) -> (f64, f64) {
    let t31131 = t3887 * t31130;
    let t31136 = 4.0_f64 * t1375 * t31096 - 6.0_f64 * t1375 * t31117 + 2.0_f64 * t1375 * t31131 - 2.0_f64 * t6958 * t6993 + t31094 + t31103 - t31106 + t31111 - t31113 + t31115 - t31122 - t31126 + t31129;
    (t31131, t31136)
}
