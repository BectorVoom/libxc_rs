//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1765;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta543(t22690: f64, t23153: f64, t23171: f64, t6561: f64, t80741: f64, t6643: f64, t23025: f64, t23030: f64, t23012: f64, t6653: f64, t22641: f64, t2588: f64, t225: f64, t814: f64, t6648: f64, t22715: f64, t6551: f64, t6640: f64, t117: f64, t4179: f64, t6559: f64, t229: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81595, t81597, t81598, t81600, t81602, t81612) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1765(t22690, t23153, t23171, t6561, t80741, t6643, t23025, t23030, t23012, t6653, t22641, t2588);
        let (t81613, t81615, t81632, t81633, t81640, t81651) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1766(t225, t814, t6648, t81612, t22715, t6551, t6640, t117, t4179, t6559, t229, t268);
    (t81595, t81597, t81598, t81600, t81602, t81612, t81613, t81615, t81632, t81633, t81640, t81651)
}
