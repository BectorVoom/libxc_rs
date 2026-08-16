//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2994/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2994(t341: f64, t79366: f64, t79386: f64, t54397: f64, t78900: f64, t15689: f64, t15700: f64, t15745: f64, t19993: f64, t225: f64, t3095: f64, t366: f64, t375: f64, t4893: f64, t53320: f64, t53328: f64, t53728: f64, t53876: f64, t53901: f64, t53955: f64, t6278: f64, t66093: f64, t66139: f64, t66141: f64, t66155: f64, t66158: f64, t66176: f64, t66215: f64, t66218: f64, t66221: f64, t66542: f64, t66777: f64, t77513: f64) -> (f64, f64, f64) {
    let t79388 = (t79366 + t79386) * t341;
    let t79395 = t78900 * t54397;
    let t79407 = 0.42874018118069736972e-3_f64 * t66093 - 0.42874018118069736972e-3_f64 * t15689 * t66777 * t4893 * t3095 - 0.22866142996303859718e-2_f64 * t66139 - 0.14291339372689912324e-3_f64 * t66141 + t66155 / 48.0_f64 - t66158 / 72.0_f64 + t53876 + t53320 * t53328 * t77513 / 16.0_f64 + 0.21437009059034868486e-3_f64 * t79388 * t225 * t366 * t375 - 0.95275595817932748825e-4_f64 * t53901 - 0.57165357490759649295e-3_f64 * t66176 + 0.25724410870841842183e-2_f64 * t15700 * t53728 * t79395 - 0.17149607247227894789e-2_f64 * t66542 * t19993 + 0.34299214494455789578e-2_f64 * t15745 * t6278 - 0.95275595817932748825e-4_f64 * t53955 - t66215 / 81.0_f64 - t66218 / 324.0_f64 - 0.85748036236139473944e-3_f64 * t66221;
    (t79388, t79395, t79407)
}
