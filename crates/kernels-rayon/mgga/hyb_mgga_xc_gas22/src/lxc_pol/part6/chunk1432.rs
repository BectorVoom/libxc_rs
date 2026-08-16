//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1432/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1432(t3740: f64, t3951: f64, t9645: f64, t9656: f64, t11474: f64, t11478: f64, t9567: f64, t11410: f64, t3957: f64, t22746: f64, t22750: f64, t2940: f64, t30689: f64, t30692: f64, t30793: f64, t3753: f64, t3757: f64, t4550: f64, t9521: f64, t9642: f64, t9654: f64, t9667: f64, t9678: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t31039 = t3740 * t3951;
    let t31040 = t31039 * t9645;
    let t31043 = t31039 * t9656;
    let t31046 = t11474 * sigma2;
    let t31050 = t11478 * sigma2;
    let t31051 = t31050 * t9567;
    let t31054 = t11410 * t3957;
    let t31055 = t31054 * t9645;
    let t31058 = t31054 * t9656;
    let t31065 = 1600.0_f64 / 27.0_f64 * t9521 * t30793 + 1408.0_f64 / 243.0_f64 * t3753 * t30692 + 704.0_f64 / 81.0_f64 * t3757 * t30689 + 12.0_f64 * t2940 * t4550 + 1408.0_f64 / 243.0_f64 * t9678 * t31040 - 1408.0_f64 / 243.0_f64 * t9667 * t31043 + 320.0_f64 * t22746 * t31046 * t9567 - 448.0_f64 * t22750 * t31051 - 512.0_f64 / 27.0_f64 * t9642 * t31055 + 512.0_f64 / 27.0_f64 * t9654 * t31058 + 1408.0_f64 / 81.0_f64 * t9642 * t31040 - 1408.0_f64 / 81.0_f64 * t9654 * t31043;
    (t31051, t31055, t31058, t31065)
}
