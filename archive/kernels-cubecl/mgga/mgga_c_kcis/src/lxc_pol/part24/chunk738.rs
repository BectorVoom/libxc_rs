//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 738/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk738<F: Float>(t144: F, t2314: F, t130: F, t3: F, t160: F, t15: F, t717: F, t787: F, t2440: F, t5: F, t88: F, t66: F, t728: F) -> (F, F, F, F, F) {
    let t9097 = t144 * t2314;
    let t9098 = t130 * t3;
    let t9099 = t9098 * t160;
    let t9102 = t15 * t717;
    let t9103 = t787 * t9102;
    let t9105 = t5 * t88 * t2440;
    let t9109 = t5 * t66 * t728;
    (t9097, t9099, t9103, t9105, t9109)
}
