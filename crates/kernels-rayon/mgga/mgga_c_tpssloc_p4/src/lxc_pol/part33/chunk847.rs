//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 847/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk847(t1294: f64, t9905: f64, t9892: f64, t3684: f64, t9467: f64, t9882: f64, t9888: f64, t9885: f64, t3824: f64, t588: f64, t1287: f64, t2225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12103 = 0.35089341735807877242e1_f64 * t1294 * t9905;
    let t12105 = 0.51947577317044391277e2_f64 * t1294 * t9892;
    let t12109 = 0.21687162600603479684e-1_f64 * t3684 * t9467;
    let t12114 = 0.32530743900905219526e-1_f64 * t3684 * t9882;
    let t12116 = 0.48159733137676571078e0_f64 * t3684 * t9888;
    let t12118 = 0.16265371950452609763e-1_f64 * t3684 * t9885;
    let t12120 = t588 * t3824;
    let t12121 = 12.0_f64 * t12120;
    let t12123 = 60.0_f64 * t2225 * t1287;
    (t12103, t12105, t12109, t12114, t12116, t12118, t12121, t12123)
}
