//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 937/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk937<F: Float>(t35458: F, t35469: F, t35475: F, t35479: F, t35484: F, t35496: F, t35502: F, t35513: F, t35515: F, t35549: F, t35552: F, t35556: F, t35594: F, t35596: F, t35608: F, t35610: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37560 = 0.4528525289702997898e-1 * t35458;
    let t37564 = 0.10289764348336736873e-1 * t35469;
    let t37566 = 0.14291339372689912324e-2 * t35475;
    let t37567 = 0.57165357490759649296e-3 * t35479;
    let t37569 = 0.28582678745379824648e-3 * t35484;
    let t37573 = 0.18868855373762491241e-1 * t35496;
    let t37576 = 0.14291339372689912324e-2 * t35502;
    let t37583 = 0.18868855373762491241e-2 * t35513;
    let t37584 = 0.12862205435420921092e-1 * t35515;
    let t37605 = 0.12579236915841660828e-2 * t35549;
    let t37606 = 0.18868855373762491241e-2 * t35552;
    let t37607 = 0.12579236915841660828e-2 * t35556;
    let t37624 = 0.85748036236139473944e-3 * t35594;
    let t37625 = 0.25724410870841842184e-2 * t35596;
    let t37631 = 0.41930789719472202758e-3 * t35608;
    let t37632 = 0.11321313224257494745e-1 * t35610;
    (t37560, t37564, t37566, t37567, t37569, t37573, t37576, t37583, t37584, t37605, t37606, t37607, t37624, t37625, t37631, t37632)
}
