//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 830/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk830<F: Float>(t125: F, t5591: F, t13926: F, t543: F, t13790: F, t1398: F, t1558: F, t836: F, t231: F, t2723: F, t136: F, t243: F) -> (F, F, F, F, F, F) {
    let t13975 = t125 * t5591;
    let t14224 = t13926 * t543;
    let t14230 = t13790 * t1398;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    let t14586 = t1558 * t2723;
    let t14587 = t14586 * t836;
    let t14685 = t243 * t136;
    (t13975, t14224, t14230, t14495, t14587, t14685)
}
