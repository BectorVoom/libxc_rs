//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1907/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1907(t22960: f64, t67128: f64, t5527: f64, t606: f64, t1408: f64, t4303: f64, t5664: f64, t868: f64, t86716: f64, t776: f64, t25373: f64, t1530: f64, t4119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97956 = t22960 * t67128;
    let t97985 = t606 * t5527;
    let t97990 = t1408 * t4303;
    let t97999 = t5664 * t868;
    let t98000 = t86716 * t97999;
    let t98003 = t5664 * t776;
    let t98004 = t25373 * t98003;
    let t98007 = t4119 * t1530;
    (t97956, t97985, t97990, t97999, t98000, t98003, t98004, t98007)
}
