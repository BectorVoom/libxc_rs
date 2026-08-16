//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta818 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2881;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta818(t2815: f64, t60160: f64, t136: f64, t59659: f64, t908: f64, t17246: f64, t699: f64, t17249: f64, t59763: f64, t59767: f64, t17252: f64, t2403: f64, t5717: f64, t2826: f64, t59676: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2881(t2815, t60160, t136, t59659, t908, t17246, t699, t17249, t59763, t59767, t17252, t2403, t5717);
        let (t60207, t60214) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2882(t136, t2826, t59676, t59661, t59663, t59665, t59670, t59674, t59678, t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204);
    (t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204, t60207, t60214)
}
