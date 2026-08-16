//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3143/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3143(t1042: f64, t1252: f64, t1266: f64, t17202: f64, t21111: f64, t21200: f64, t21272: f64, t21275: f64, t24664: f64, t3711: f64, t44174: f64, t5304: f64, t5391: f64, t69719: f64, t82543: f64, t82550: f64, t82553: f64, t82555: f64, t82560: f64, t82565: f64) -> f64 {
    let t82570 = 0.85748036236139473944e-3_f64 * t3711 * t1042 * t17202 * t82543 + 0.24136484273876296368e-1_f64 * t21272 * t5304 - 0.45732285992607719436e-2_f64 * t82550 + 0.14291339372689912324e-3_f64 * t82553 - 0.34299214494455789577e-2_f64 * t82555 * t1252 + 0.12862205435420921092e-2_f64 * t44174 * t24664 - 0.30488190661738479624e-2_f64 * t82560 + 0.10162730220579493208e-1_f64 * t5391 * t21111 + 0.85748036236139473944e-3_f64 * t69719 - 0.14291339372689912324e-3_f64 * t82565 * t1266 + 0.25724410870841842184e-2_f64 * t21275 * t21200;
    t82570
}
