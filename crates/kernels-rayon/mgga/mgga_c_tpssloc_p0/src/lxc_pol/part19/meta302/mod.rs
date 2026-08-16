//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1088;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta302(t3791: f64, t562: f64, t10: f64, t2229: f64, t116: f64, t117: f64, t556: f64, t252: f64, t2631: f64, t243: f64, t828: f64, t852: f64, t3034: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22740, t22811, t22815, t22843, t22997, t23076, t23175) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1088(t3791, t562, t10, t2229, t116, t117, t556, t252, t2631, t243, t828, t852);
        let t23508 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1089(t3034, t371);
    (t22740, t22811, t22815, t22843, t22997, t23076, t23175, t23508)
}
