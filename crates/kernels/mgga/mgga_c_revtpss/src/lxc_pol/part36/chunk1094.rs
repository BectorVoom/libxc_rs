//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1094/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1094<F: Float>(t1882: F, t6843: F, t221: F, t22852: F, t13790: F, t543: F, t23087: F, t47672: F, t23059: F, t4147: F, t1774: F, t471: F, t843: F, t1962: F, t41154: F, t25373: F, t25392: F) -> (F, F, F, F, F, F, F, F, F) {
    let t85659 = t6843 * t1882;
    let t85776 = t221 * t22852;
    let t86413 = t13790 * t6843;
    let t86641 = t85659 * t543;
    let t86791 = t23087 * t47672;
    let t86825 = t23059 * t4147;
    let t91338 = t471 * t1774;
    let t92612 = 1232.0 / 27.0 * t843;
    let t92742 = t1962 * t41154;
    let t92837 = t25373 * t25392;
    (t85776, t86413, t86641, t86791, t86825, t91338, t92612, t92742, t92837)
}
