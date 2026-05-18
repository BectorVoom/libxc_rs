//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 871/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk871<F: Float>(t2777: F, t5759: F, t2439: F, t136: F, t1883: F, t2457: F, t10139: F, t1892: F, t4086: F, t786: F, t2470: F, t5740: F) -> (F, F, F, F) {
    let t14202 = t2777 * t5759;
    let t14203 = t2439 * t14202;
    let t14219 = t1883 * t136;
    let t14220 = t14219 * t2457;
    let t14221 = t10139 * t14220;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14242 = t5740 * t2470;
    (t14203, t14221, t14239, t14242)
}
