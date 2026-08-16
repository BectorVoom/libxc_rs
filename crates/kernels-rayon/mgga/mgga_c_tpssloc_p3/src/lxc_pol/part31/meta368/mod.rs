//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1301;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1302;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta368(t13109: f64, t13113: f64, t5398: f64, t751: f64, t707: f64, t13133: f64, t1462: f64, t2427: f64, t5597: f64, t9922: f64, t13124: f64, t5522: f64, t67: f64, t758: f64, t3966: f64, t4195: f64, t4194: f64, t184: f64, t5392: f64, t607: f64, t12939: f64, t13121: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16710) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1301(t13109, t13113, t5398, t751, t707, t13133, t1462, t2427, t5597, t9922, t13124, t5522, t67);
        let (t16712, t16715, t16719, t16720) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1302(t16710, t758, t3966, t4195, t4194, t184, t5392, t607, t12939, t13121, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t9853, t9859, t9894, t9907, t9921);
    (t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16715, t16719, t16720)
}
