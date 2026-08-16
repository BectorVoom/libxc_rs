//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1293/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293(t2928: f64, t315: f64, t2931: f64, t10843: f64, t923: f64, t10744: f64, t10750: f64, t10760: f64, t10765: f64, t10771: f64, t10825: f64, t2861: f64, t2881: f64, t2886: f64, t2888: f64, t2907: f64, t41827: f64, t41987: f64, t41998: f64, t42002: f64, t42005: f64, t42011: f64, t42020: f64, t42025: f64, t42031: f64, t42097: f64, t42105: f64, t42106: f64, t932: f64, t933: f64, t952: f64) -> (f64, f64, f64) {
    let t42109 = t2928 * t2928;
    let t42110 = 1.0_f64 / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    let t42113 = 1.0_f64 / t42112;
    let t42117 = t10843 * t923;
    let t42122 = t41998 + t42002 - t42005 + 24.0_f64 * t10765 * t10744 - 24.0_f64 * t10771 * t41987 * t932 - 6.0_f64 * t2861 * t42011 * t932 + 0.96491876992155210402e2_f64 * t2886 * t42011 * t2888 + 0.14035736694323150897e2_f64 * t10825 * t10750 - 0.70178683471615754484e1_f64 * t42020 * t2907 - t42025 + t42031 - t42097 - t42105 + 0.23392894490538584828e1_f64 * t42106 * t952 + 0.91082604192152556044e5_f64 * t42111 * t41827 * t42113 + 4.0_f64 * t42117 * t933 + 6.0_f64 * t10760 * t2881;
    (t42110, t42113, t42122)
}
