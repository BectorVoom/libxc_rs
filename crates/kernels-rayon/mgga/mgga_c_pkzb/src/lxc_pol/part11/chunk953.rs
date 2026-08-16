//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 953/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk953(t1429: f64, t4803: f64, t15: f64, t20: f64, t399: f64, t3329: f64, t983: f64, t4810: f64, t2499: f64, t3333: f64, t27: f64, t10415: f64, t10418: f64, t23: f64, t28: f64, t3324: f64, t3330: f64, t3334: f64, t7: f64, t980: f64, t984: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10421 = -t1429 - t4803;
    let t10422 = 3.0_f64 * t10421;
    let t10423 = t15 * t10422;
    let t10427 = 1.0_f64 / t20 / t399;
    let t10428 = sigma2 * t10427;
    let t10437 = t3329 * t983;
    let t10438 = t4810 * t10437;
    let t10441 = t2499 * t3333;
    let t10444 = -t10422;
    let t10445 = t27 * t10444;
    let t10448 = -10.0_f64 / 27.0_f64 * t7 * t10415 + 10.0_f64 / 3.0_f64 * t7 * t10418 + 5.0_f64 / 3.0_f64 * t7 * t10423 - 1232.0_f64 / 27.0_f64 * t10428 * t28 + 440.0_f64 / 9.0_f64 * t3324 * t984 - 80.0_f64 / 9.0_f64 * t980 * t3330 - 40.0_f64 / 3.0_f64 * t980 * t3334 - 10.0_f64 / 27.0_f64 * t23 * t10438 + 10.0_f64 / 3.0_f64 * t23 * t10441 + 5.0_f64 / 3.0_f64 * t23 * t10445;
    (t10421, t10422, t10423, t10428, t10437, t10438, t10441, t10444, t10445, t10448)
}
