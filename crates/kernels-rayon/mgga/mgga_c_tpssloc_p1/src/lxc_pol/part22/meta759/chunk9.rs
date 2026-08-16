//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2558/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2558(t1117: f64, t21723: f64, t44075: f64, t44077: f64, t11310: f64, t11415: f64, t1155: f64, t15126: f64, t15136: f64, t1682: f64, t18603: f64, t18606: f64, t18622: f64, t18643: f64, t21845: f64, t21906: f64, t21939: f64, t21942: f64, t3357: f64, t3376: f64, t3401: f64, t43692: f64, t44155: f64, t44223: f64, t4819: f64, t4857: f64, t63502: f64, t71672: f64, t71697: f64, t71700: f64, t71704: f64, t71707: f64) -> (f64, f64) {
    let t71711 = 0.24955700379505800916e5_f64 * t44075 * t21723 * t44077 * t1117;
    let t71712 = -0.12304822629859687989e5_f64 * t44155 * t21942 * t1155 - 0.11696447245269292414e1_f64 * t3376 * t21939 * t1155 + 0.17315859105681463759e2_f64 * t3401 * t71672 * t1155 + 0.30762056574649219974e4_f64 * t11310 * t18622 * t4857 + 0.91082604192152556044e5_f64 * t44223 * t21906 * t43692 * t1155 + 0.10526802520742363173e2_f64 * t15126 * t18603 - 0.70178683471615754484e1_f64 * t15136 * t18606 + 0.96491876992155210402e2_f64 * t11415 * t21845 + 0.96491876992155210402e2_f64 * t3357 * t63502 * t1682 + 0.96491876992155210402e2_f64 * t3357 * t18643 * t4819 + t71697 + t71700 - t71704 - t71707 - t71711;
    (t71711, t71712)
}
