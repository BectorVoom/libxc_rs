//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1109/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1109(t231: f64, t836: f64, t886: f64, t25392: f64, t1950: f64, t867: f64, t786: f64, t2467: f64, t233: f64, t1949: f64, t7056: f64, t10073: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25394 = t886 * t836 * t231;
    let t25395 = t25392 * t25394;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    let t25400 = t25399 * t2467;
    let t25402 = t867 * t233;
    let t25403 = t25402 * t1949;
    let t25404 = t7056 * t25403;
    let t25406 = 0.24093411633903331839e-3_f64 * t10073 * t25404;
    (t25394, t25395, t25398, t25399, t25400, t25402, t25403, t25404, t25406)
}
