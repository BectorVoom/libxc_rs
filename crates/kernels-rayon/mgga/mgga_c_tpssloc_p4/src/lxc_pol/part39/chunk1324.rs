//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1324/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1324(t111: f64, t8283: f64, t110363: f64, t12521: f64, t12524: f64, t12813: f64, t16521: f64, t16524: f64, t16541: f64, t20173: f64, t2199: f64, t2319: f64, t2363: f64, t30112: f64, t30125: f64, t30315: f64, t30363: f64, t30382: f64, t30385: f64, t30390: f64, t3941: f64, t5376: f64, t55353: f64, t55571: f64, t671: f64, t8189: f64, t8207: f64, t8212: f64, t8273: f64, t8294: f64) -> f64 {
    let t111246 = t8283 * t111;
    let t111284 = 54.0_f64 * t12524 * t30385 + 27.0_f64 * t111246 * t2319 + 0.135e2_f64 * t8207 * t12813 + 27.0_f64 * t30112 * t16541 + 54.0_f64 * t55353 * t8212 + 27.0_f64 * t16521 * t8189 + 54.0_f64 * t16524 * t30125 + 54.0_f64 * t12524 * t30382 + 0.135e2_f64 * t30363 * t2363 + 54.0_f64 * t110363 * t5376 + 54.0_f64 * t12524 * t30390 + 27.0_f64 * t3941 * t2199 * t12813 + 0.135e2_f64 * t12521 * t8273 + 27.0_f64 * t55571 * t8294 + 54.0_f64 * t20173 * t30382 + 54.0_f64 * t20173 * t30385 + 54.0_f64 * t3941 * t30315 * t671 + 27.0_f64 * t3941 * t8273 * t2363;
    t111284
}
