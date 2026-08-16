//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1283/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1283(t2470: f64, t5740: f64, t4101: f64, t1432: f64, t5763: f64, t1385: f64, t5710: f64, t1904: f64, t3899: f64, t689: f64, t3920: f64, t5603: f64) -> (f64, f64, f64, f64, f64) {
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14255 = t1385 * t5710;
    let t14274 = t3899 * t1904;
    let t14276 = 0.10975748638225852664e-1_f64 * t689 * t14274;
    let t14280 = t5603 * t3920;
    (t14243, t14252, t14255, t14276, t14280)
}
