//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1293;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1294;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta230(t831: f64, t9671: f64, t2617: f64, t2638: f64, t116: f64, t126: f64, t136: f64, t16: f64, t2386: f64, t625: f64, t2385: f64, t686: f64, t781: f64, t685: f64, t120: f64, t118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9672, t9674, t9688, t9689, t9691, t9692, t9694) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1293(t831, t9671, t2617, t2638, t116, t126, t136, t16, t2386, t625, t2385, t686, t781);
        let (t9695, t9697) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1294(t685, t9694, t120, t781);
        let t9698 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1295(t118, t9697);
    (t9672, t9674, t9688, t9689, t9691, t9692, t9694, t9695, t9697, t9698)
}
