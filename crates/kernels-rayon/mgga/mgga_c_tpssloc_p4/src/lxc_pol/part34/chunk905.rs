//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 905/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk905(t14276: f64, t21259: f64, t21263: f64, t21265: f64, t21267: f64, t21270: f64, t21302: f64, t21305: f64, t21306: f64, t21309: f64, t21312: f64, t21317: f64, t21320: f64, t21321: f64, t21336: f64, t21348: f64, t21360: f64, t2861: f64, t2886: f64, t2905: f64, t2930: f64, t311: f64, t5743: f64) -> f64 {
    let t21363 = -6.0_f64 * t14276 * t5743 + 6.0_f64 * t2886 * t21259 - t21263 - t21265 - t21267 + t21270 - t21302 - t21305 + 0.96491876992155210402e2_f64 * t2886 * t21306 - 0.35089341735807877242e1_f64 * t2905 * t21309 + 0.51947577317044391277e2_f64 * t2930 * t21312 + t21317 - t21320 - 6.0_f64 * t2861 * t21321 + t21336 - 0.19751673498613801407e-1_f64 * t21348 - 0.310907e-1_f64 * t21360 * t311;
    t21363
}
