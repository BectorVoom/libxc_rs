//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1304/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1304<F: Float>(t26894: F, t26921: F, t1294: F, t471: F, t355: F, t1204: F, t7627: F, t26884: F, t460: F, t1210: F, t3627: F, t5457: F) -> (F, F, F, F, F, F) {
    let t96927 = t26894 * t26921;
    let t96928 = t471 * t1294;
    let t96929 = t355 * t96928;
    let t96933 = t1204 * t7627;
    let t96938 = t460 * t26884;
    let t96953 = t1210 * t26921;
    let t96954 = t3627 * t5457;
    (t96927, t96929, t96933, t96938, t96953, t96954)
}
