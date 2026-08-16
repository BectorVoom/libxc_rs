//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 972/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk972(t3: f64, t8372: f64, t1518: f64, t8295: f64, t117: f64, t8362: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64) {
    let t8373 = t3 * t8372;
    let t8377 = param_d * t8372;
    let t8383 = t8295 * t1518;
    let t8386 = t117 * t8362;
    let t8389 = 3.0_f64 * t1916 * t2189 + 3.0_f64 * t1918 * t2187 + 6.0_f64 * t572 * t8383 + 3.0_f64 * t572 * t8386 + t573 * t8377;
    (t8373, t8377, t8383, t8386, t8389)
}
