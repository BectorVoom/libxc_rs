//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1743/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1743<F: Float>(t1398: F, t281: F, t543: F, t68: F, t10139: F, t1357: F, t4078: F, t689: F, t1445: F, t3899: F, t10115: F, t562: F) -> (F, F, F, F, F, F, F) {
    let t10142 = t281 * t68 * t1398 * t543;
    let t10143 = t10139 * t10142;
    let t10150 = t1357 * t4078;
    let t10151 = t689 * t10150;
    let t10153 = t3899 * t1445;
    let t10154 = t689 * t10153;
    let t10157 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t562;
    (t10142, t10143, t10150, t10151, t10153, t10154, t10157)
}
