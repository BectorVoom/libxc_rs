//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2918/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2918<F: Float>(t14473: F, t4489: F, t2906: F, t42110: F, t42113: F, t5774: F, t959: F, t10629: F, t14259: F, t5790: F, t10623: F, t5812: F) -> (F, F, F, F) {
    let t60816 = F::cast_from(0.46785788981077169656e1_f64) * t14473 * t4489;
    let t60821 = F::cast_from(0.91082604192152556044e5_f64) * t959 * t42110 * t5774 * t42113 * t2906;
    let t60825 = F::cast_from(0.10254018858216406658e4_f64) * t959 * t10629 * t5790 * t14259;
    let t60827 = F::cast_from(0.17315859105681463759e2_f64) * t10623 * t5812;
    (t60816, t60821, t60825, t60827)
}
