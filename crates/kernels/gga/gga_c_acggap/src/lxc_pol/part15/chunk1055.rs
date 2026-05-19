//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1055/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1055<F: Float>(t35484: F, t35496: F, t35502: F, t35513: F, t35515: F, t35549: F, t35552: F, t35556: F, t35594: F, t35596: F, t35608: F, t35610: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37569 = F::cast_from(0.28582678745379824648e-3_f64) * t35484;
    let t37573 = F::cast_from(0.18868855373762491241e-1_f64) * t35496;
    let t37576 = F::cast_from(0.14291339372689912324e-2_f64) * t35502;
    let t37583 = F::cast_from(0.18868855373762491241e-2_f64) * t35513;
    let t37584 = F::cast_from(0.12862205435420921092e-1_f64) * t35515;
    let t37605 = F::cast_from(0.12579236915841660828e-2_f64) * t35549;
    let t37606 = F::cast_from(0.18868855373762491241e-2_f64) * t35552;
    let t37607 = F::cast_from(0.12579236915841660828e-2_f64) * t35556;
    let t37624 = F::cast_from(0.85748036236139473944e-3_f64) * t35594;
    let t37625 = F::cast_from(0.25724410870841842184e-2_f64) * t35596;
    let t37631 = F::cast_from(0.41930789719472202758e-3_f64) * t35608;
    let t37632 = F::cast_from(0.11321313224257494745e-1_f64) * t35610;
    (t37569, t37573, t37576, t37583, t37584, t37605, t37606, t37607, t37624, t37625, t37631, t37632)
}
