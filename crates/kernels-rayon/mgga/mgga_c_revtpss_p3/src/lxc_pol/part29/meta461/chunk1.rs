//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1714/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1714(t26292: f64, t7284: f64, t25878: f64, t26234: f64, t1445: f64, t7492: f64, t689: f64, t1385: f64, t2097: f64) -> (f64, f64, f64, f64, f64) {
    let t26294 = 0.96373646535613327357e-2_f64 * t7284 * t26292;
    let t26295 = t25878 * t26234;
    let t26301 = t7492 * t1445;
    let t26302 = t689 * t26301;
    let t26304 = t1385 * t2097;
    (t26294, t26295, t26301, t26302, t26304)
}
