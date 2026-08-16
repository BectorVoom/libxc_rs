//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1144/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1144<F: Float>(t30159: F, t36213: F, t7586: F, t2041: F, t4777: F, t4781: F, t4787: F, t2030: F, t2288: F, t4262: F, t839: F, t1089: F, t4643: F, t598: F, t7533: F) -> (F, F, F, F, F, F) {
    let t36302 = t30159 * t7586 * t36213;
    let t36306 = t2041 * t4777;
    let t36308 = t2041 * t4781;
    let t36310 = t2041 * t4787;
    let t36314 = t2030 * t4262 * t2288 * t839;
    let t36320 = t598 * t1089 * t4643 * t7533;
    (t36302, t36306, t36308, t36310, t36314, t36320)
}
