//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta568(t25810: f64, t7553: f64, t5685: f64, t6690: f64, t6689: f64, t1922: f64, t5844: f64, t1052: f64, t1635: f64, t1920: f64, t25450: f64, t25736: f64, t25755: f64, t25778: f64, t28470: f64, t28475: f64, t28481: f64, t28485: f64, t28488: f64, t28492: f64, t28496: f64, t28500: f64, t28505: f64, t388: f64, t4660: f64, t6687: f64, t7600: f64, t7625: f64, t3: f64, t5398: f64, t1933: f64, t1618: f64, t1622: f64, t1937: f64, t23447: f64, t23537: f64, t23541: f64, t25577: f64, t25580: f64, t25598: f64, t25616: f64, t25618: f64, t25625: f64, t25629: f64, t25645: f64, t5857: f64, t5861: f64, t5869: f64, t5875: f64, t5880: f64, t6755: f64, t6765: f64, t7583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28510, t28515, t28516, t28519, t28523) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1936(t25810, t7553, t5685, t6690, t6689, t1922, t5844, t1052, t1635, t1920, t25450, t25736, t25755, t25778, t28470, t28475, t28481, t28485, t28488, t28492, t28496, t28500, t28505, t388, t4660, t6687, t7600, t7625);
        let (t28525, t28526, t28550) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1937(t3, t5398, t1933, t1618, t1622, t1937, t23447, t23537, t23541, t25577, t25580, t25598, t25616, t25618, t25625, t25629, t25645, t5857, t5861, t5869, t5875, t5880, t6755, t6765, t7583);
    (t28510, t28515, t28516, t28519, t28523, t28525, t28526, t28550)
}
