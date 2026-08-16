//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1350/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1350(t22986: f64, t23270: f64, t2553: f64, t857: f64, t865: f64, t23196: f64, t23204: f64, t6562: f64, t22979: f64, t2597: f64, t82150: f64, t82154: f64, t82156: f64, t82161: f64, t82165: f64, t82169: f64, t82172: f64, t82174: f64) -> f64 {
    let t82179 = t22986 * t23270 * t857 * t2553 * t865;
    let t82182 = t6562 * t23204 * t23196;
    let t82186 = 0.11514538467937585055e0_f64 * t82150 - t82154 - 0.24674011002723396548e-1_f64 * t82156 + 0.9869604401089358619e-1_f64 * t82161 - 0.82246703342411321825e-2_f64 * t82165 - 0.16449340668482264365e-1_f64 * t82169 + 0.24674011002723396548e-1_f64 * t82172 + 0.23029076935875170111e0_f64 * t82174 + 0.49348022005446793095e-1_f64 * t82179 - 0.24674011002723396548e-1_f64 * t82182 + 12.0_f64 * t2597 * t22979;
    t82186
}
