//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1366;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta251(t10213: f64, t10216: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64, t3038: f64, t2775: f64, t283: f64, t61: f64, t2770: f64, t976: f64, t3185: f64, t3199: f64, t1014: f64, t10471: f64, t10470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10942, t10947, t10949, t10952, t10969) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1366(t10213, t10216, t3030, t990, t3032, t3129, t3038, t2775, t283);
        let (t10970, t10996, t11034, t11037, t11045, t11046) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1367(t10969, t61, t2770, t976, t10947, t3185, t3199, t1014, t10471, t10470);
    (t10942, t10949, t10952, t10969, t10970, t10996, t11034, t11037, t11045, t11046)
}
