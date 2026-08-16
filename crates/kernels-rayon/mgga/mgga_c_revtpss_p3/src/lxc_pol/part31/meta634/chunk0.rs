//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2088/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2088(t15749: f64, t7117: f64, t25490: f64, t4845: f64, t15666: f64, t27479: f64, t3215: f64, t25577: f64, t4817: f64, t15711: f64, t7132: f64, t15655: f64, t1972: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100329 = t7117 * t15749;
    let t100332 = 0.57165357490759649296e-3_f64 * t25490 * t4845;
    let t100334 = 0.57165357490759649296e-3_f64 * t7117 * t15666;
    let t100336 = 0.57165357490759649296e-3_f64 * t27479 * t3215;
    let t100342 = 0.20325460441158986416e-2_f64 * t25577 * t4817;
    let t100343 = t7132 * t15711;
    let t100345 = t15655 * t1972;
    (t100329, t100332, t100334, t100336, t100342, t100343, t100345)
}
