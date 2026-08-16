//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3141/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3141(t11570: f64, t17686: f64, t1174: f64, t15269: f64, t15274: f64, t15288: f64, t18420: f64, t3447: f64, t3449: f64, t3469: f64, t44487: f64, t460: f64, t4889: f64, t4900: f64, t4934: f64, t6138: f64, t64969: f64, t64976: f64, t64979: f64, t64981: f64, t64988: f64, t64990: f64) -> f64 {
    let t64994 = t11570 * t17686;
    let t65001 = -0.55555555555555555554e-3_f64 * t64969 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t6138 * t3469 * t460 - 0.18106995884773662551e-2_f64 * t64976 + 0.6172839506172839506e-4_f64 * t64979 + 0.2962962962962962963e-2_f64 * t64981 + 0.88888888888888888888e-2_f64 * t4889 * t15269 + 0.44444444444444444444e-2_f64 * t4889 * t15274 - 0.18518518518518518518e-3_f64 * t64988 - t44487 + 0.88888888888888888886e-2_f64 * t3447 * t4900 * t64990 + 0.33333333333333333332e-2_f64 * t3447 * t3449 * t64994 + 0.55555555555555555554e-3_f64 * t3447 * t18420 * t15288;
    t65001
}
