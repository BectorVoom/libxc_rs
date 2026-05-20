//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2665/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2665<F: Float>(t13821: F, t13999: F, t13716: F, t1413: F, t547: F, t807: F, t550: F, t9794: F, t14224: F, t9793: F, t13928: F, t9962: F) -> (F, F, F, F, F) {
    let t49062 = t13999 * t13821;
    let t49066 = t807 * t547 * t1413 * t13716;
    let t49068 = t9794 * t550;
    let t49070 = t9793 * t49068 * t14224;
    let t49071 = F::cast_from(0.13553694749236397037e-4_f64) * t49070;
    let t49085 = t9962 * t13928;
    (t49062, t49066, t49068, t49071, t49085)
}
