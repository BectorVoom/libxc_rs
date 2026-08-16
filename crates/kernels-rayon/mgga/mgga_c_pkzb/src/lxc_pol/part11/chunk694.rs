//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 694/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk694(t4952: f64, t541: f64, t1585: f64, t518: f64, t101: f64, t109: f64, t1588: f64, t4906: f64, t106: f64, t1589: f64, t4929: f64, t4934: f64, t4937: f64, t4939: f64, t4943: f64, t4945: f64, t4947: f64, t4950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4953 = t4952 * t541;
    let t4957 = 1.0_f64 / t1585 / t518;
    let t4958 = t101 * t4957;
    let t4960 = 1.0_f64 / t1588 / t109;
    let t4961 = t4906 * t4960;
    let t4965 = 1.0_f64 / t1585 / t106;
    let t4966 = t101 * t4965;
    let t4967 = t4906 * t1589;
    let t4978 = -0.47063e1_f64 * t4929 + 0.31375333333333333334e1_f64 * t4934 - 0.36604555555555555556e1_f64 * t4937 - 0.16068111111111111111e1_f64 * t4939 + 0.28051666666666666666e0_f64 * t4943 - 0.56103333333333333332e0_f64 * t4945 - 0.6545388888888888889e0_f64 * t4947 - 0.46308888888888888888e0_f64 * t4950;
    (t4953, t4957, t4958, t4960, t4961, t4965, t4966, t4967, t4978)
}
