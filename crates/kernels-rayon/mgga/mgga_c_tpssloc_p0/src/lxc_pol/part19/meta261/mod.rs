//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1009;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta261(t11708: f64, t3505: f64, t10469: f64, t466: f64, t10471: f64, t1208: f64, t478: f64, t10477: f64, t483: f64, t1215: f64, t3507: f64, t3508: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11709, t11712, t11713, t11714, t11715, t11716, t11717, t11718, t11719, t11720) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1009(t11708, t3505, t10469, t466, t10471, t1208, t478, t10477, t483, t1215, t3507);
        let t11721 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1010(t3508, t475);
    (t11709, t11712, t11713, t11714, t11715, t11716, t11717, t11718, t11719, t11720, t11721)
}
