//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1395/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1395(t10993: f64, t6717: f64, t10981: f64, t10984: f64, t10325: f64, t10932: f64, t10944: f64, t10988: f64, t1935: f64, t23422: f64, t23489: f64, t23504: f64, t3143: f64, t3148: f64, t3153: f64, t343: f64, t360: f64, t6734: f64, t82987: f64, t82990: f64, t83134: f64, t83139: f64, t83142: f64, t83153: f64, t83157: f64) -> f64 {
    let t83159 = t6717 * t10993;
    let t83165 = t6717 * t10981;
    let t83167 = t6717 * t10984;
    let t83171 = -t6717 * t10932 / 36.0_f64 + 0.48447307312968469026e-2_f64 * t83134 + 7.0_f64 / 648.0_f64 * t6717 * t10944 - 0.60559134141210586284e-3_f64 * t83139 + 0.10093189023535097714e-3_f64 * t82987 * t83142 * t82990 * t360 + 0.30279567070605293142e-3_f64 * t23489 * t23504 - 0.10093189023535097714e-3_f64 * t1935 * t10325 * t343 * t6734 - t83153 / 54.0_f64 + t23422 * t3153 / 18.0_f64 - t83157 / 432.0_f64 - t83159 / 144.0_f64 - t23422 * t3143 / 36.0_f64 - t23422 * t3148 / 27.0_f64 + t83165 / 288.0_f64 + t83167 / 216.0_f64 + t6717 * t10988 / 288.0_f64;
    t83171
}
