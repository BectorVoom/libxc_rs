//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 993/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk993<F: Float>(t116: F, t22746: F, t14586: F, t6016: F, t1558: F, t231: F, t221: F, t23279: F, t22648: F, t602: F, t1882: F, t6843: F, t22852: F, t13790: F, t543: F, t23087: F, t47672: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75941 = t22746 * t116;
    let t76106 = t14586 * t6016;
    let t76161 = t6016 * t1558 * t231;
    let t76613 = t221 * t23279;
    let t85037 = t22648 * t602;
    let t85659 = t6843 * t1882;
    let t85776 = t221 * t22852;
    let t86413 = t13790 * t6843;
    let t86641 = t85659 * t543;
    let t86791 = t23087 * t47672;
    (t75941, t76106, t76161, t76613, t85037, t85776, t86413, t86641, t86791)
}
