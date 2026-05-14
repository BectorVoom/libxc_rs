//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 753/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk753<F: Float>(t1234: F, t5390: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t1260: F, t5326: F, t17376: F, t3599: F, t1285: F, t17395: F, t1781: F, t697: F, t1222: F) -> (F, F, F, F, F, F, F) {
    let t17505 = t1234 * t5390;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17569 = t5326 * t1260;
    let t17572 = t17376 * t3599;
    let t17605 = t1285 * t17395;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    (t17505, t17525, t17529, t17569, t17572, t17605, t17629)
}
