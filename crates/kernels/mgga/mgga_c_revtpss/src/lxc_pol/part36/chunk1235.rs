//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1235/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1235<F: Float>(t42859: F, t487: F, t1276: F, t2148: F, t13038: F, t2142: F, t26894: F, t26921: F, t1210: F, t29193: F, t26948: F, t8945: F) -> (F, F, F, F, F, F, F) {
    let t96886 = t487 * t42859;
    let t96888 = t2148 * t96886 * t1276;
    let t96889 = t13038 * t2142;
    let t96927 = t26894 * t26921;
    let t96953 = t1210 * t26921;
    let t96979 = t1210 * t29193;
    let t96986 = t26894 * t29193;
    let t97040 = t26948 * t487;
    let t97041 = t97040 * t8945;
    (t96888, t96889, t96927, t96953, t96979, t96986, t97041)
}
