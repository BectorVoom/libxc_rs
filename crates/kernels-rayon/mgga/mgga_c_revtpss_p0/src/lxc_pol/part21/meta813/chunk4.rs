//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2981/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2981(t15994: f64, t3241: f64, t43537: f64, t53668: f64, t11933: f64, t16035: f64, t11774: f64, t127: f64, t15585: f64, t4872: f64, t11247: f64, t11631: f64, t11662: f64, t11697: f64, t11703: f64, t15691: f64, t15696: f64, t15700: f64, t16027: f64, t16089: f64, t3117: f64, t42310: f64, t42695: f64, t42788: f64, t43082: f64, t43297: f64, t4573: f64, t4907: f64, t53670: f64, t53822: f64) -> f64 {
    let t54314 = t3241 * t15994;
    let t54316 = t43537 * t53668;
    let t54324 = t11933 * t16035;
    let t54341 = t11774 * t127 * t4872 * t15585;
    let t54346 = -2.0_f64 / 81.0_f64 * t54314 - 0.77173232612525526552e-2_f64 * t54316 * t3117 * t53670 * t11631 * t11247 - 0.21722835846488666732e-1_f64 * t42695 * t4907 + 0.45732285992607719436e-2_f64 * t54324 - 0.68598428988911579154e-2_f64 * t43297 * t16027 - 0.42874018118069736972e-3_f64 * t11774 * t15696 * t11697 - 0.85748036236139473944e-3_f64 * t43082 * t15696 * t11662 + 0.14291339372689912324e-2_f64 * t16089 * t11703 * t4573 * t53822 + 0.30488190661738479624e-2_f64 * t42788 - 0.57165357490759649295e-3_f64 * t54341 - 0.85748036236139473944e-3_f64 * t15700 * t15691 * t42310;
    t54346
}
