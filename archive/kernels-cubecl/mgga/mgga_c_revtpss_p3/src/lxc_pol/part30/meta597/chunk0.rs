//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2058/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2058<F: Float>(t26884: F, t460: F, t1210: F, t26921: F, t3627: F, t5457: F, t26983: F, t7635: F, t29193: F, t26894: F, t3566: F, t7627: F) -> (F, F, F, F, F, F, F) {
    let t96938 = t460 * t26884;
    let t96953 = t1210 * t26921;
    let t96954 = t3627 * t5457;
    let t96966 = t26983 * t7635;
    let t96979 = t1210 * t29193;
    let t96986 = t26894 * t29193;
    let t97019 = t3566 * t7627;
    (t96938, t96953, t96954, t96966, t96979, t96986, t97019)
}
