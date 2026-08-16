//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1255/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1255(t2453: f64, t2458: f64, t7049: f64, t1950: f64, t2769: f64, t786: f64, t10997: f64, t231: f64, t2645: f64, t886: f64, t25404: f64, t40270: f64) -> (f64, f64, f64, f64) {
    let t93252 = t2453 * t7049 * t2458;
    let t93261 = t786 * t1950 * t2769;
    let t93262 = t93261 * t10997;
    let t93267 = t886 * t2645 * t231;
    let t93272 = 0.96373646535613327356e-3_f64 * t40270 * t25404;
    (t93252, t93262, t93267, t93272)
}
