//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3208/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3208<F: Float>(t12772: F, t17729: F, t17731: F, t3718: F, t44546: F, t5353: F, t3588: F, t5245: F, t45833: F, t58919: F, t1042: F, t1250: F, t12784: F, t12787: F, t12858: F, t12920: F, t13043: F, t16756: F, t17420: F, t17426: F, t17625: F, t17633: F, t17638: F, t17646: F, t17709: F, t17711: F, t17736: F, t17760: F, t17784: F, t20921: F, t3363: F, t3617: F, t3625: F, t3626: F, t3711: F, t3720: F, t44624: F, t44938: F, t471: F, t5331: F, t56861: F, t57536: F, t58921: F, t59159: F, t59162: F, t59173: F, t59176: F, t59179: F) -> (F, F) {
    let t59182 = t17729 * t12772 * t17731;
    let t59185 = t3718 * t44546 * t5353;
    let t59186 = F::cast_from(0.14291339372689912324e-3_f64) * t59185;
    let t59187 = t5245 * t3588;
    let t59196 = t45833 * t58919;
    let t59215 = F::cast_from(0.12862205435420921092e-2_f64) * t44624 * t17625 + F::cast_from(0.38586616306262763275e-2_f64) * t17709 * t3720 * t57536 * t17711 + F::cast_from(0.25724410870841842184e-2_f64) * t59159 - F::cast_from(0.12862205435420921092e-2_f64) * t59162 * t12858 - F::cast_from(0.85748036236139473944e-3_f64) * t44938 + F::cast_from(0.25724410870841842183e-2_f64) * t17426 * t17420 - F::cast_from(0.64311027177104605458e-3_f64) * t5331 * t3720 * t16756 * t17784 - F::cast_from(0.42874018118069736972e-3_f64) * t59173 - F::cast_from(0.85748036236139473944e-3_f64) * t59176 + F::cast_from(0.42874018118069736972e-3_f64) * t59179 + F::cast_from(0.11433071498151929859e-2_f64) * t59182 + t59186 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t59187 * t1250 + F::cast_from(0.14291339372689912324e-2_f64) * t17736 * t12787 * t20921 * t12920 - F::cast_from(0.21437009059034868486e-3_f64) * t59196 * t3720 * t58921 * t13043 * t471 - F::cast_from(0.14291339372689912324e-2_f64) * t56861 * t17760 - F::cast_from(0.85748036236139473944e-3_f64) * t12784 * t17646 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t17633 * t17638 - F::cast_from(0.7145669686344956162e-3_f64) * t3711 * t1042 * t3617 * t5245 * t3363;
    (t59187, t59215)
}
