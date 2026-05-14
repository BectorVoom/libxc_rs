//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 758/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk758<F: Float>(t4147: F, t5778: F, t1882: F, t4003: F, t136: F, t550: F, t220: F, t221: F, t5627: F, t1398: F, t125: F, t5591: F, t543: F, t1558: F, t836: F, t231: F) -> (F, F, F, F, F, F, F, F) {
    let t13648 = t5778 * t4147;
    let t13790 = t1882 * t4003;
    let t13846 = t550 * t136;
    let t13847 = t13846 * t220;
    let t13877 = t221 * t5627;
    let t13926 = t1882 * t1398;
    let t13975 = t125 * t5591;
    let t14224 = t13926 * t543;
    let t14230 = t13790 * t1398;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    (t13648, t13846, t13847, t13877, t13975, t14224, t14230, t14495)
}
