//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 783/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk783<F: Float>(t2130: F, t7922: F, t861: F, t2140: F, t3054: F, t609: F, t865: F, t14651: F, t159: F, t448: F, t7911: F, t2137: F, t3035: F, t2132: F, t309: F, t7886: F) -> (F, F, F, F, F, F) {
    let t30009 = t7922 * t861 * t2130;
    let t30011 = 0.52041769129231196772e1 * t30009 * t2140;
    let t30015 = 0.39512695097613069591e1 * t3054 * t609 * t865;
    let t30023 = t14651 * t159;
    let t30028 = t7911 * t448;
    let t30029 = t2137 * t30028;
    let t30032 = t3035 * t2130;
    let t30036 = 0.15612530738769359031e2 * t30032 * t2132 * t7886 * t309;
    (t30011, t30015, t30023, t30028, t30029, t30036)
}
