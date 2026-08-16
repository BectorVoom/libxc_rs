//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1001/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1001<F: Float>(t16710: F, t758: F, t3966: F, t4195: F, t4194: F, t184: F, t5392: F, t607: F, t12939: F, t13121: F, t16699: F, t16700: F, t16703: F, t16705: F, t16707: F, t16708: F, t16709: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F) -> (F, F, F, F) {
    let t16711 = t16710 * t758;
    let t16712 = F::cast_from(0.18311447306006545054e-3_f64) * t16711;
    let t16713 = t4195 * t3966;
    let t16715 = F::cast_from(24.0_f64) * t4194 * t16713;
    let t16716 = t184 * t5392;
    let t16717 = t16716 * t607;
    let t16719 = F::cast_from(24.0_f64) * t12939 * t16717;
    let t16720 = -t9894 + t16699 + t9907 - t16700 + t16703 + t9853 + t16705 + t16707 - t13121 - t9921 - t16708 + t16709 - t16712 + t16715 + t16719 + t9859;
    (t16712, t16715, t16719, t16720)
}
