//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2221/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2221(t1058: f64, t27467: f64, t100255: f64, t100261: f64, t100262: f64, t100268: f64, t100270: f64, t100272: f64, t15887: f64, t16186: f64, t1972: f64, t25526: f64, t3130: f64, t375: f64, t4797: f64, t4869: f64, t4875: f64, t7122: f64, t7125: f64, t93764: f64) -> f64 {
    let t100275 = 0.57165357490759649296e-3_f64 * t27467 * t1058;
    let t100282 = -0.57165357490759649296e-3_f64 * t100255 * t3130 - 0.45732285992607719436e-2_f64 * t25526 * t4869 + t100261 - 0.76220476654346199061e-3_f64 * t100262 + 0.42874018118069736972e-3_f64 * t7122 * t16186 - 0.57165357490759649296e-3_f64 * t93764 * t4875 + 0.3811023832717309953e-3_f64 * t100268 - 0.30488190661738479624e-2_f64 * t100270 - 0.95275595817932748827e-4_f64 * t100272 + t100275 - 0.45732285992607719436e-2_f64 * t4797 * t7125 * t375 + 0.42874018118069736972e-3_f64 * t15887 * t1972 * t375;
    t100282
}
