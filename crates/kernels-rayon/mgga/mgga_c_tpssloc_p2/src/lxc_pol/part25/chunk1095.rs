//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1095/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1095(t1369: f64, t80866: f64, t22782: f64, t3777: f64, t22783: f64, t3876: f64, t22788: f64, t12361: f64, t6952: f64, t15: f64, t2229: f64, t1361: f64, t192: f64, t1995: f64, t22690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    let t80870 = t80869 * t1369;
    let t80872 = t22783 * t3876;
    let t80876 = t22788 * t3876;
    let t80878 = t6952 * t12361;
    let t80881 = 1.0_f64 / t2229 / t15;
    let t80885 = t80881 * t1995 * t192 * t22690 * t1361;
    (t80867, t80870, t80872, t80876, t80878, t80881, t80885)
}
