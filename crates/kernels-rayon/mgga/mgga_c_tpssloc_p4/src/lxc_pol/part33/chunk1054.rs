//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1054/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1054(t11779: f64, t21758: f64, t248: f64, t1230: f64, t21776: f64, t21769: f64, t1156: f64, t21906: f64, t3400: f64, t1164: f64, t4869: f64, t6106: f64) -> (f64, f64, f64, f64, f64) {
    let t22208 = t248 * t11779 * t21758;
    let t22214 = t248 * t1230 * t21776;
    let t22218 = t248 * t1230 * t21769;
    let t22222 = t3400 * t21906 * t1156;
    let t22224 = 0.35089341735807877242e1_f64 * t1164 * t22222;
    let t22226 = 0.51947577317044391276e2_f64 * t4869 * t6106;
    (t22208, t22214, t22218, t22224, t22226)
}
