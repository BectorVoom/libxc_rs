//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2303/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2303(t25: f64, t3701: f64, t6463: f64, t15909: f64, t5127: f64, t5187: f64, t11987: f64, t6305: f64, t3704: f64, t5397: f64, t1298: f64, t16557: f64, t2219: f64, t5170: f64, t606: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t19596 = t6463 * t3701;
    let t19599 = 0.21687162600603479684e-1_f64 * t15909;
    let t19603 = t5127 * t5187;
    let t19606 = t11987 * t6305;
    let t19611 = t3704 * t5397;
    let t19617 = piecewise3(t26, 0.0_f64, 8.0_f64 / 27.0_f64 * t19606 * t606 - 8.0_f64 / 9.0_f64 * t5170 * t2219 - 2.0_f64 / 9.0_f64 * t19611 * t606 + 2.0_f64 / 3.0_f64 * t1298 * t16557);
    (t19596, t19599, t19603, t19606, t19611, t19617)
}
