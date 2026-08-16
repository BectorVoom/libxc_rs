//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk757;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta149(t1155: f64, t3403: f64, t3439: f64, t60: f64, t461: f64, t3448: f64, t457: f64, t974: f64, t1229: f64, t3247: f64, t1215: f64, t3508: f64, t3242: f64, t3584: f64, t3612: f64, t1932: f64, t475: f64, t671: f64, t88: f64, t193: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4883, t4899, t4900, t4908, t4934, t4972, t4978) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk757(t1155, t3403, t3439, t60, t461, t3448, t457, t974, t1229, t3247, t1215, t3508);
        let (t4987, t5068, t5079, t5113, t5126) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk758(t3242, t3584, t1215, t3612, t1932, t475, t671, t88, t193, t531);
    (t4883, t4899, t4900, t4908, t4934, t4972, t4978, t4987, t5068, t5079, t5113, t5126)
}
