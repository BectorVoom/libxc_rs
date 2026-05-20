//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2256/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2256<F: Float>(t60221: F, t6957: F, t13269: F, t607: F, t13272: F, t25105: F, t10309: F, t28126: F, t1493: F, t2248: F, t77: F, t1928: F, t25099: F, t25102: F, t25106: F, t25110: F, t25157: F, t25159: F, t28081: F, t28127: F, t28133: F, t6960: F, t6963: F, t7706: F, t7720: F, t92666: F, t92699: F) -> F {
    let t101320 = t60221 * t6957;
    let t101323 = t13269 * t607;
    let t101326 = t13272 * t25105;
    let t101333 = t10309 * t28126;
    let t101337 = t77 * t1493 * t2248;
    let t101340 = F::new(5.0) / F::new(3.0) * t28127 * t25110 + F::new(2.0) / F::new(3.0) * t6963 * t28081 + F::new(5.0) / F::new(3.0) * t25099 * t28133 + F::new(2.0) / F::new(3.0) * t25102 * t7720 + F::new(5.0) / F::new(3.0) * t25106 * t28133 + F::new(5.0) / F::new(3.0) * t101320 * t6960 + F::new(2.0) / F::new(3.0) * t101323 * t1928 + F::new(5.0) / F::new(3.0) * t101326 * t6960 - F::new(5.0) / F::new(3.0) * t92666 * t7706 + F::new(5.0) / F::new(6.0) * t92699 * t7706 - F::new(5.0) * t101333 * t25159 - F::new(5.0) * t25157 * t101337;
    t101340
}
