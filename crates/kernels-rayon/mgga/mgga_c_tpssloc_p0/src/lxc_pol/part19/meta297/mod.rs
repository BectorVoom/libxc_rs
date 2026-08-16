//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1079;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta297(t776: f64, t868: f64, t10189: f64, t344: f64, t134: f64, t2978: f64, t10213: f64, t60: f64, t135: f64, t340: f64, t6733: f64, t884: f64, t122: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13487, t13779, t13783, t13784, t13797, t13798, t13822, t13831) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1079(t776, t868, t10189, t344, t134, t2978, t10213, t60, t135, t340, t6733, t884);
        let t13969 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1080(t122, t247);
    (t13487, t13779, t13783, t13784, t13797, t13798, t13822, t13831, t13969)
}
