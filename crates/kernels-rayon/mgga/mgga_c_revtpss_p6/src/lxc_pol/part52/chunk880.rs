//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 880/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk880(t2435: f64, t7493: f64, t26069: f64, t26277: f64, t26072: f64, t7515: f64, t116: f64, t7356: f64) -> (f64, f64, f64, f64) {
    let t26363 = 0.73171657588172351096e-2_f64 * t2435 * t7493;
    let t26365 = 0.22849835011101738147e-2_f64 * t26069 * t26277;
    let t26366 = t26072 * t7515;
    let t26399 = t7356 * t116;
    (t26363, t26365, t26366, t26399)
}
