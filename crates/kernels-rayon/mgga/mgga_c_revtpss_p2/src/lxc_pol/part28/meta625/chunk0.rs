//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2223/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2223(t7105: f64, t816: f64, t15670: f64, t1972: f64, t4857: f64, t7125: f64, t25495: f64, t4845: f64, t15749: f64, t7117: f64, t25490: f64, t15666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100315 = t7105 * t816;
    let t100321 = t15670 * t1972;
    let t100324 = t4857 * t7125;
    let t100327 = t25495 * t4845;
    let t100329 = t7117 * t15749;
    let t100332 = 0.57165357490759649296e-3_f64 * t25490 * t4845;
    let t100334 = 0.57165357490759649296e-3_f64 * t7117 * t15666;
    (t100315, t100321, t100324, t100327, t100329, t100332, t100334)
}
