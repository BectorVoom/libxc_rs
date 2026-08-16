//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1143/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1143(t25: f64, t6665: f64, t28: f64, t6504: f64, t8307: f64, t8513: f64, t3701: f64, t6995: f64, t2314: f64, t8327: f64, t4034: f64, t1266: f64, t8326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30767 = t25 * t6665;
    let t30974 = t28 * t6665;
    let t31019 = t8513 * t8307 * t6504;
    let t31035 = t3701 * t6995;
    let t31054 = t2314 * t8327;
    let t31055 = 2.0_f64 * t31054;
    let t31056 = t4034 * t8327;
    let t31057 = 2.0_f64 * t31056;
    let t31058 = t1266 * t8326;
    (t30767, t30974, t31019, t31035, t31055, t31057, t31058)
}
