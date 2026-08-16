//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk890;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta237(t5686: f64, t690: f64, t2770: f64, t5398: f64, t5689: f64, t892: f64, t3216: f64, t5946: f64, t10595: f64, t5698: f64, t10599: f64, t5717: f64, t699: f64, t5720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t17175 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk890(t5686, t690);
        let (t17177, t17195, t17202, t17210, t17218, t17286, t17288) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk891(t2770, t5398, t5689, t892, t3216, t5946, t10595, t5698, t10599, t5717, t699, t5720);
    (t17175, t17177, t17195, t17202, t17210, t17218, t17286, t17288)
}
