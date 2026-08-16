//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1404/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1404(t15045: f64, t689: f64, t4470: f64, t786: f64, t789: f64, t4534: f64, t779: f64, t2435: f64, t4322: f64, t1596: f64, t2873: f64, t1614: f64, t2942: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15047 = 0.10975748638225852664e-1_f64 * t689 * t15045;
    let t15048 = t786 * t4470;
    let t15050 = 0.19514881078765566038e-1_f64 * t15048 * t789;
    let t15060 = t779 * t4534;
    let t15062 = 0.10975748638225852664e-1_f64 * t689 * t15060;
    let t15063 = t2435 * t4322;
    let t15101 = t1596 * t2873;
    let t15104 = t1614 * t2942;
    (t15047, t15050, t15062, t15063, t15101, t15104)
}
