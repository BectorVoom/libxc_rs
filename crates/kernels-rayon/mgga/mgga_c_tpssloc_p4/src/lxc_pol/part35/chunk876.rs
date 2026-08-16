//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 876/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk876(t28: f64, t528: f64, t1294: f64, t9722: f64, t9919: f64, t9905: f64, t9892: f64, t3684: f64, t9467: f64, t9882: f64, t9888: f64, t9885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12072 = 1.0_f64 / t528 / t28;
    let t12087 = 0.10389515463408878255e3_f64 * t1294 * t9722;
    let t12094 = 0.35089341735807877242e1_f64 * t1294 * t9919;
    let t12103 = 0.35089341735807877242e1_f64 * t1294 * t9905;
    let t12105 = 0.51947577317044391277e2_f64 * t1294 * t9892;
    let t12109 = 0.21687162600603479684e-1_f64 * t3684 * t9467;
    let t12114 = 0.32530743900905219526e-1_f64 * t3684 * t9882;
    let t12116 = 0.48159733137676571078e0_f64 * t3684 * t9888;
    let t12118 = 0.16265371950452609763e-1_f64 * t3684 * t9885;
    (t12072, t12087, t12094, t12103, t12105, t12109, t12114, t12116, t12118)
}
