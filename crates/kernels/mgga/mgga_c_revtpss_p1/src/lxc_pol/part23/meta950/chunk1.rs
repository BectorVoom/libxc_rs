//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3143/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3143<F: Float>(t1042: F, t1252: F, t1266: F, t17202: F, t21111: F, t21200: F, t21272: F, t21275: F, t24664: F, t3711: F, t44174: F, t5304: F, t5391: F, t69719: F, t82543: F, t82550: F, t82553: F, t82555: F, t82560: F, t82565: F) -> F {
    let t82570 = F::cast_from(0.85748036236139473944e-3_f64) * t3711 * t1042 * t17202 * t82543 + F::cast_from(0.24136484273876296368e-1_f64) * t21272 * t5304 - F::cast_from(0.45732285992607719436e-2_f64) * t82550 + F::cast_from(0.14291339372689912324e-3_f64) * t82553 - F::cast_from(0.34299214494455789577e-2_f64) * t82555 * t1252 + F::cast_from(0.12862205435420921092e-2_f64) * t44174 * t24664 - F::cast_from(0.30488190661738479624e-2_f64) * t82560 + F::cast_from(0.10162730220579493208e-1_f64) * t5391 * t21111 + F::cast_from(0.85748036236139473944e-3_f64) * t69719 - F::cast_from(0.14291339372689912324e-3_f64) * t82565 * t1266 + F::cast_from(0.25724410870841842184e-2_f64) * t21275 * t21200;
    t82570
}
