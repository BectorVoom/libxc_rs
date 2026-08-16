//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 974/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk974(t2099: f64, t2918: f64, t757: f64, t2946: f64, t300: f64, t2107: f64, t1123: f64, t779: f64, t2029: f64, t759: f64, t2106: f64, t178: f64, t5711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7692 = t2099 * t2918;
    let t7694 = 0.28582678745379824648e-3_f64 * t757 * t7692;
    let t7695 = t300 * t2946;
    let t7696 = t7695 * t2107;
    let t7699 = t779 * t1123;
    let t7700 = t300 * t7699;
    let t7701 = t2029 * t759;
    let t7702 = t7701 * t2106;
    let t7703 = t7700 * t7702;
    let t7706 = t5711 * t178;
    (t7692, t7694, t7695, t7696, t7699, t7700, t7701, t7702, t7703, t7706)
}
