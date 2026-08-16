//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 896/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk896(t1053: f64, t3215: f64, t390: f64, t1376: f64) -> (f64, f64, f64, f64) {
    let t10163 = t1053 * t1053;
    let t10164 = 1.0_f64 / t10163;
    let t11094 = 1.0_f64 / t3215 / t390;
    let t12019 = t1376 * t1376;
    let t12020 = 1.0_f64 / t12019;
    (t10164, t11094, t12019, t12020)
}
