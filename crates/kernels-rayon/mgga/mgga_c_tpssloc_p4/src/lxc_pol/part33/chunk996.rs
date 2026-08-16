//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 996/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk996(t21256: f64, t21363: f64, t300: f64, t21348: f64, t4483: f64, t5804: f64, t17954: f64, t4475: f64, t959: f64, t4488: f64, t5791: f64, t1637: f64, t5950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21365 = t300 * (t21256 + t21363);
    let t21367 = 0.19751673498613801407e-1_f64 * t300 * t21348;
    let t21369 = 0.35089341735807877242e1_f64 * t4483 * t5804;
    let t21370 = t17954 * t4475;
    let t21372 = 0.51947577317044391277e2_f64 * t959 * t21370;
    let t21373 = t4488 * t5791;
    let t21375 = 0.35089341735807877242e1_f64 * t959 * t21373;
    let t21376 = t5950 * t1637;
    (t21365, t21367, t21369, t21372, t21375, t21376)
}
