//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1097/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1097(t1937: f64, t34251: f64, t7359: f64, t7735: f64, t1501: f64, t1936: f64) -> (f64, f64, f64) {
    let t34253 = 2.0_f64 * t34251 * t1937;
    let t34255 = 2.0_f64 * t7359 * t7735;
    let t34258 = t1501 * t1936;
    (t34253, t34255, t34258)
}
