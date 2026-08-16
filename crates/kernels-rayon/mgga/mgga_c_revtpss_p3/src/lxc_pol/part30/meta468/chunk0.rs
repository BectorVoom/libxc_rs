//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1775/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1775(t25304: f64, t7057: f64, t25301: f64, t1032: f64, t860: f64, t867: f64) -> (f64, f64, f64, f64) {
    let t25305 = t25304 * t7057;
    let t25307 = 0.22849835011101738147e-2_f64 * t25305 * t25301;
    let t25308 = t860 * t1032;
    let t25309 = t25308 * t867;
    (t25305, t25307, t25308, t25309)
}
