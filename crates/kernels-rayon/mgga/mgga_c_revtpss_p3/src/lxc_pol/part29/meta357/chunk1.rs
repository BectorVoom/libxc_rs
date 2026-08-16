//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1288/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1288(t12854: f64, t5330: f64, t11772: f64, t3623: f64, t3717: f64, t1263: f64, t675: f64, t1122: f64, t247: f64, t1261: f64, t126: f64, t3617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12855 = t12854 * t5330;
    let t12865 = t3623 * t11772;
    let t12866 = t3717 * t12865;
    let t12879 = t675 * t1263;
    let t12881 = t247 * t12879 * t1122;
    let t12882 = t1261 * t12881;
    let t12884 = t126 * t3617;
    (t12855, t12865, t12866, t12879, t12882, t12884)
}
