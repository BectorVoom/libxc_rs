//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 906/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk906(t10199: f64, t10202: f64, t10208: f64, t1036: f64, t13691: f64, t13696: f64, t13699: f64, t13744: f64, t13747: f64, t13750: f64, t13783: f64, t1670: f64, t245: f64, t2944: f64, t2952: f64, t3078: f64, t3081: f64, t4625: f64, t4647: f64, t4654: f64, t934: f64) -> f64 {
    let t13786 = 3.0_f64 / 16.0_f64 * t10199 * t13691 - t10202 * t4647 / 4.0_f64 - t3078 * t13696 / 4.0_f64 - t3078 * t13699 / 8.0_f64 + t10208 * t1670 / 4.0_f64 + t3081 * t4625 / 2.0_f64 + t1036 * t13744 / 4.0_f64 - t13747 * t2944 / 8.0_f64 + t13750 * t934 / 2.0_f64 + t4654 * t2952 / 4.0_f64 + t245 * t13783 / 2.0_f64;
    t13786
}
