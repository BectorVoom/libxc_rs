//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2999/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2999(t15731: f64, t3169: f64, t11281: f64, t11774: f64, t11883: f64, t1469: f64, t15707: f64, t15725: f64, t15804: f64, t16149: f64, t3241: f64, t372: f64, t42926: f64, t42929: f64, t42932: f64, t42947: f64, t42962: f64, t4801: f64, t4916: f64, t54398: f64) -> f64 {
    let t54733 = t3169 * t15731;
    let t54735 = -11.0_f64 / 54.0_f64 * t11883 * t4916 - t3241 * t15804 / 6.0_f64 - 0.28582678745379824648e-3_f64 * t42926 - 0.28582678745379824648e-3_f64 * t42929 + 0.14291339372689912324e-3_f64 * t42932 + 0.85748036236139473944e-3_f64 * t15725 * t16149 + 0.17149607247227894789e-2_f64 * t11774 * t372 * t4801 * t1469 * t54398 - 0.42874018118069736972e-3_f64 * t42947 - 0.42874018118069736972e-3_f64 * t15707 * t11281 + 0.57165357490759649295e-3_f64 * t42962 + 0.7622047665434619906e-3_f64 * t54733;
    t54735
}
