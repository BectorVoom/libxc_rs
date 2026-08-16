//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2949/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2949(t11466: f64, t11507: f64, t11554: f64, t15413: f64, t1634: f64, t19021: f64, t19294: f64, t19297: f64, t23711: f64, t23761: f64, t23785: f64, t2987: f64, t3012: f64, t4707: f64, t4708: f64, t52443: f64, t6190: f64, t6205: f64, t78303: f64, t78305: f64, t78307: f64, t78309: f64, t78311: f64, t78313: f64, t78315: f64, t972: f64) -> f64 {
    let t78316 = -0.70178683471615754484e1_f64 * t15413 * t19294 - 0.31168546390226634765e3_f64 * t52443 * t19297 - 0.14035736694323150897e2_f64 * t11466 * t23711 * t972 + 0.10526802520742363173e2_f64 * t3012 * t6190 * t4707 + 0.6233709278045326953e3_f64 * t11507 * t23785 * t972 - 0.35089341735807877242e1_f64 * t11554 * t23761 - 0.35089341735807877242e1_f64 * t2987 * t4708 * t6205 - 0.35089341735807877242e1_f64 * t2987 * t1634 * t19021 + t78303 - t78305 + t78307 - t78309 + t78311 - t78313 - t78315;
    t78316
}
