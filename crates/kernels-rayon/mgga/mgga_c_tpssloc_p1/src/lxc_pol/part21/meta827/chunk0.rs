//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2918/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2918(t14473: f64, t4489: f64, t2906: f64, t42110: f64, t42113: f64, t5774: f64, t959: f64, t10629: f64, t14259: f64, t5790: f64, t10623: f64, t5812: f64) -> (f64, f64, f64, f64) {
    let t60816 = 0.46785788981077169656e1_f64 * t14473 * t4489;
    let t60821 = 0.91082604192152556044e5_f64 * t959 * t42110 * t5774 * t42113 * t2906;
    let t60825 = 0.10254018858216406658e4_f64 * t959 * t10629 * t5790 * t14259;
    let t60827 = 0.17315859105681463759e2_f64 * t10623 * t5812;
    (t60816, t60821, t60825, t60827)
}
