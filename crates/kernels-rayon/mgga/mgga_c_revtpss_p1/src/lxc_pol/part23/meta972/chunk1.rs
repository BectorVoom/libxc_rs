//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3292/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3292(t1882: f64, t6888: f64, t22857: f64, t555: f64, t1399: f64, t46505: f64, t5675: f64, t5745: f64, t5755: f64, t75021: f64, t75024: f64, t75026: f64, t75035: f64, t75039: f64, t75041: f64, t75049: f64, t75053: f64) -> (f64, f64, f64) {
    let t86441 = t6888 * t1882;
    let t86445 = t555 * t22857;
    let t86453 = 0.39029762157531132076e-1_f64 * t75021 - 0.58544643236296698113e-1_f64 * t75024 + 0.19514881078765566037e-2_f64 * t75026 - 0.17563392970889009434e0_f64 * t75035 + 0.17563392970889009434e0_f64 * t75039 + 0.39512695097613069592e1_f64 * t5745 * t86441 * t5675 - 0.65854491829355115987e0_f64 * t5755 * t86445 * t1399 - 0.29272321618148349057e-1_f64 * t75041 + 0.46263278077393568556e-2_f64 * t46505 + 0.98781737744032673976e-1_f64 * t75049 - 0.98781737744032673976e-1_f64 * t75053;
    (t86441, t86445, t86453)
}
