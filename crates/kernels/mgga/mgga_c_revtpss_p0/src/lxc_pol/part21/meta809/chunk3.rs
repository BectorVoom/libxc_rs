//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2956/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2956<F: Float>(t11710: F, t16089: F, t16090: F, t3059: F, t606: F, t11883: F, t4924: F, t2258: F, t999: F, t11703: F, t11991: F, t15584: F, t15968: F, t16095: F, t1656: F, t3092: F, t42499: F, t42506: F, t42516: F, t42537: F, t42721: F, t43082: F, t4573: F, t4578: F, t4808: F, t4873: F) -> (F, F, F) {
    let t53820 = t16089 * t11710 * t16090;
    let t53822 = t606 * t3059;
    let t53832 = t11883 * t4924;
    let t53835 = t2258 * t999;
    let t53844 = F::cast_from(0.71456696863449561621e-3_f64) * t11991 * t4808 + F::cast_from(0.11433071498151929859e-2_f64) * t53820 - F::cast_from(0.17149607247227894789e-2_f64) * t16089 * t3092 * t4578 * t53822 + t42499 / F::new(864.0) + F::new(7.0) / F::new(1944.0) * t42506 - t42516 / F::new(108.0) - F::new(77.0) / F::new(486.0) * t42721 * t1656 + F::new(11.0) / F::new(324.0) * t53832 + F::cast_from(0.42344709252414555035e-3_f64) * t42537 - F::cast_from(0.71456696863449561621e-3_f64) * t16095 * t11703 * t4573 * t53835 - F::cast_from(0.85748036236139473944e-3_f64) * t43082 * t15584 * t4873 * t15968;
    (t53822, t53835, t53844)
}
