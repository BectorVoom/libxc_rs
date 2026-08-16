//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1445;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta380<F: Float>(t13109: F, t13113: F, t5398: F, t751: F, t707: F, t13133: F, t1462: F, t2427: F, t5597: F, t9922: F, t13124: F, t5522: F, t67: F, t758: F, t3966: F, t4195: F, t4194: F, t184: F, t5392: F, t607: F, t12939: F, t13121: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16710) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1445::<F>(t13109, t13113, t5398, t751, t707, t13133, t1462, t2427, t5597, t9922, t13124, t5522, t67);
        let (t16712, t16715, t16719, t16720) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1446::<F>(t16710, t758, t3966, t4195, t4194, t184, t5392, t607, t12939, t13121, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t9853, t9859, t9894, t9907, t9921);
    (t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16715, t16719, t16720)
}
