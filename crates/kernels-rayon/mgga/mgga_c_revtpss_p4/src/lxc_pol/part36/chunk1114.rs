//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1114/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1114(t25304: f64, t7057: f64, t25301: f64, t11007: f64, t233: f64, t2470: f64, t7059: f64, t7064: f64, t1949: f64, t785: f64, t780: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25305 = t25304 * t7057;
    let t25307 = 0.22849835011101738147e-2_f64 * t25305 * t25301;
    let t25317 = t11007 * t233;
    let t25331 = t7059 * t2470;
    let t25333 = 0.17135234354032049604e-1_f64 * t7064 * t25331;
    let t25334 = t785 * t1949;
    let t25335 = t25334 * t780;
    let t25337 = 0.65049603595885220126e-3_f64 * t2439 * t25335;
    (t25305, t25307, t25317, t25331, t25333, t25334, t25335, t25337)
}
