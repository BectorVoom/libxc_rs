//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta823 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2893;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta823(t14379: f64, t49226: f64, t2791: f64, t5689: f64, t2794: f64, t4433: f64, t2792: f64, t2836: f64, t5727: f64, t10661: f64, t17520: f64, t2793: f64, t2842: f64, t10704: f64, t5726: f64, t10702: f64, t13654: f64, t4399: f64, t17527: f64, t42100: f64, t42102: f64, t5694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60354, t60359, t60360, t60371, t60374) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2893(t14379, t49226, t2791, t5689, t2794, t4433, t2792, t2836, t5727, t10661, t17520, t2793);
        let (t60377, t60381, t60384, t60387, t60391) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2894(t17520, t2836, t2842, t10704, t5726, t10702, t2793, t13654, t4399, t17527, t42100, t42102, t5694);
    (t60354, t60359, t60360, t60371, t60374, t60377, t60381, t60384, t60387, t60391)
}
