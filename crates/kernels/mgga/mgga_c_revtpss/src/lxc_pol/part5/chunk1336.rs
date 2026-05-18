//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1336/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1336<F: Float>(t20721: F, t247: F, t3719: F, t3670: F, t5390: F, t1225: F, t18281: F, t1012: F, t1010: F, t5843: F, t5378: F, t5381: F) -> (F, F, F, F, F) {
    let t21200 = t247 * t3719 * t20721;
    let t21203 = t3670 * t5390;
    let t21209 = t1225 * t18281;
    let t21210 = t1012 * t21209;
    let t21213 = t5843 * t1010;
    let t21216 = t5381 * t5378;
    (t21200, t21203, t21210, t21213, t21216)
}
