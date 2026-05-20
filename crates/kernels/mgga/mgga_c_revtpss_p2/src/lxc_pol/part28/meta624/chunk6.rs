//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2220/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2220<F: Float>(t4857: F, t7131: F, t16163: F, t7122: F, t15772: F, t7132: F, t15984: F, t25517: F, t1058: F, t27464: F, t3201: F, t7801: F) -> (F, F, F, F, F, F) {
    let t100255 = t4857 * t7131;
    let t100261 = F::cast_from(0.57165357490759649296e-3_f64) * t7122 * t16163;
    let t100262 = t7132 * t15772;
    let t100268 = t25517 * t15984;
    let t100270 = t27464 * t1058;
    let t100272 = t7801 * t3201;
    (t100255, t100261, t100262, t100268, t100270, t100272)
}
