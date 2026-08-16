//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2956/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2956(t11710: f64, t16089: f64, t16090: f64, t3059: f64, t606: f64, t11883: f64, t4924: f64, t2258: f64, t999: f64, t11703: f64, t11991: f64, t15584: f64, t15968: f64, t16095: f64, t1656: f64, t3092: f64, t42499: f64, t42506: f64, t42516: f64, t42537: f64, t42721: f64, t43082: f64, t4573: f64, t4578: f64, t4808: f64, t4873: f64) -> (f64, f64, f64) {
    let t53820 = t16089 * t11710 * t16090;
    let t53822 = t606 * t3059;
    let t53832 = t11883 * t4924;
    let t53835 = t2258 * t999;
    let t53844 = 0.71456696863449561621e-3_f64 * t11991 * t4808 + 0.11433071498151929859e-2_f64 * t53820 - 0.17149607247227894789e-2_f64 * t16089 * t3092 * t4578 * t53822 + t42499 / 864.0_f64 + 7.0_f64 / 1944.0_f64 * t42506 - t42516 / 108.0_f64 - 77.0_f64 / 486.0_f64 * t42721 * t1656 + 11.0_f64 / 324.0_f64 * t53832 + 0.42344709252414555035e-3_f64 * t42537 - 0.71456696863449561621e-3_f64 * t16095 * t11703 * t4573 * t53835 - 0.85748036236139473944e-3_f64 * t43082 * t15584 * t4873 * t15968;
    (t53822, t53835, t53844)
}
