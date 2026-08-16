//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 301/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk301(t439: f64, t10: f64, t1034: f64, t14: f64, t237: f64, t800: f64, t1035: f64, t1037: f64) -> (f64, f64, f64, f64) {
    let t1039 = f64::sqrt(t439);
    let t1040 = t1039 * t10;
    let t1041 = t1040 * t1034;
    let t1044 = t237 * t14 * t800;
    let t1046 = -0.632975e0_f64 * t1035 - 0.29896666666666666667e0_f64 * t1037 - 0.1023875e0_f64 * t1041 - 0.82156666666666666667e-1_f64 * t1044;
    (t1040, t1041, t1044, t1046)
}
