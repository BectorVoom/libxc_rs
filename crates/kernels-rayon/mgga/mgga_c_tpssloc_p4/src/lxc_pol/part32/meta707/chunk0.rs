//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2208/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2208(t1408: f64, t4303: f64, t5664: f64, t868: f64, t86716: f64, t776: f64, t25373: f64, t1530: f64, t4119: f64, t22960: f64, t5660: f64, t67164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97990 = t1408 * t4303;
    let t97999 = t5664 * t868;
    let t98000 = t86716 * t97999;
    let t98003 = t5664 * t776;
    let t98004 = t25373 * t98003;
    let t98007 = t4119 * t1530;
    let t98008 = t22960 * t98007;
    let t98011 = t5660 * t776;
    let t98012 = t22960 * t98011;
    let t98015 = t22960 * t67164;
    (t97990, t97999, t98000, t98003, t98004, t98007, t98008, t98011, t98012, t98015)
}
