//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1489/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1489(t1041: f64, t11622: f64, t3172: f64, t12021: f64, t3173: f64, t1032: f64, t1040: f64, t11902: f64, t11762: f64, t3241: f64, t1047: f64, t11659: f64, t11703: f64, t11705: f64, t11714: f64, t11883: f64, t3177: f64, t3238: f64, t3248: f64, t3255: f64, t42216: f64, t42227: f64, t4892: f64, t4899: f64) -> f64 {
    let t42230 = t1041 * t3172 * t11622;
    let t42232 = t12021 * t3173;
    let t42235 = t11902 * t1032 * t1040;
    let t42240 = t3241 * t11762;
    let t42246 = 0.28582678745379824648e-2_f64 * t4892 * t11703 * t11659 * t42216 - 0.14291339372689912324e-2_f64 * t4899 * t11703 * t11659 * t11705 - 0.91464571985215438872e-2_f64 * t11714 * t3177 + 0.17149607247227894789e-2_f64 * t42227 + 0.57165357490759649296e-3_f64 * t42230 + 0.17149607247227894789e-2_f64 * t42232 + 0.85748036236139473944e-3_f64 * t42235 * t1047 - 11.0_f64 / 27.0_f64 * t11883 * t3238 + 2.0_f64 / 27.0_f64 * t42240 + 11.0_f64 / 54.0_f64 * t11883 * t3248 + 22.0_f64 / 81.0_f64 * t11883 * t3255;
    t42246
}
