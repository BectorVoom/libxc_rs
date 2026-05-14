//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1111/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1111<F: Float>(t1210: F, t26921: F, t29193: F, t26894: F, t26948: F, t487: F, t8945: F, t3736: F, t7635: F, t3566: F, t13036: F, t13040: F, t7616: F, t12854: F, t29096: F, t11772: F, t26865: F) -> (F, F, F, F, F, F, F, F) {
    let t96953 = t1210 * t26921;
    let t96979 = t1210 * t29193;
    let t96986 = t26894 * t29193;
    let t97040 = t26948 * t487;
    let t97041 = t97040 * t8945;
    let t97065 = t7635 * t3736;
    let t97066 = t3566 * t97065;
    let t97133 = t13036 * t7616 * t13040;
    let t97149 = t12854 * t29096;
    let t97173 = t26865 * t11772;
    (t96953, t96979, t96986, t97041, t97066, t97133, t97149, t97173)
}
