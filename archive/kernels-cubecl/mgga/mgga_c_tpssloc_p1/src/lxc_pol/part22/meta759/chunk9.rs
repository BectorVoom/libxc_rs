//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2558/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2558<F: Float>(t1117: F, t21723: F, t44075: F, t44077: F, t11310: F, t11415: F, t1155: F, t15126: F, t15136: F, t1682: F, t18603: F, t18606: F, t18622: F, t18643: F, t21845: F, t21906: F, t21939: F, t21942: F, t3357: F, t3376: F, t3401: F, t43692: F, t44155: F, t44223: F, t4819: F, t4857: F, t63502: F, t71672: F, t71697: F, t71700: F, t71704: F, t71707: F) -> (F, F) {
    let t71711 = F::cast_from(0.24955700379505800916e5_f64) * t44075 * t21723 * t44077 * t1117;
    let t71712 = -F::cast_from(0.12304822629859687989e5_f64) * t44155 * t21942 * t1155 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t21939 * t1155 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t71672 * t1155 + F::cast_from(0.30762056574649219974e4_f64) * t11310 * t18622 * t4857 + F::cast_from(0.91082604192152556044e5_f64) * t44223 * t21906 * t43692 * t1155 + F::cast_from(0.10526802520742363173e2_f64) * t15126 * t18603 - F::cast_from(0.70178683471615754484e1_f64) * t15136 * t18606 + F::cast_from(0.96491876992155210402e2_f64) * t11415 * t21845 + F::cast_from(0.96491876992155210402e2_f64) * t3357 * t63502 * t1682 + F::cast_from(0.96491876992155210402e2_f64) * t3357 * t18643 * t4819 + t71697 + t71700 - t71704 - t71707 - t71711;
    (t71711, t71712)
}
