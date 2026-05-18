//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 914/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk914<F: Float>(t221: F, t6836: F, t9921: F, t3978: F, t125: F, t6816: F, t6843: F, t13848: F, t6869: F, t9818: F, t9816: F, t1413: F) -> (F, F, F, F, F, F, F) {
    let t22068 = t9921 * t221 * t6836;
    let t22069 = t3978 * t22068;
    let t22074 = t125 * t6816;
    let t22079 = t125 * t6843;
    let t22102 = t9818 * t13848 * t6869;
    let t22103 = t9816 * t22102;
    let t22125 = t1413 * t6816;
    (t22068, t22069, t22074, t22079, t22102, t22103, t22125)
}
