//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 849/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk849<F: Float>(t5: F, t10309: F, t33358: F, t38: F, t8911: F, t2247: F, t7574: F, t8441: F, t8621: F, t32132: F, t32138: F, t32145: F, t32156: F, t8737: F, t8913: F, t117: F, t32172: F, t32174: F, t32176: F, t32178: F, t32828: F, t32830: F, t32832: F, t33346: F, t670: F, t8564: F) -> (F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t33359 = t10309 * t33358;
    let t33362 = t38 * t8911;
    let t33363 = t2247 * t33362;
    let t33367 = t8621 * t8441 * t7574;
    let t33370 = t2247 * t33358;
    let t33374 = piecewise3(t8, 0.0, 5.0 / 144.0 * t32132 * t8913 - 5.0 / 24.0 * t33359 * t32138 - 5.0 / 36.0 * t33363 * t32145 + 5.0 / 72.0 * t8737 * t33367 + 5.0 / 72.0 * t33370 * t32156);
    let t33375 = t33374 * t117;
    let t33381 = 2.0 * t33346 * t670 + t32172 + t32174 + t32176 + t32178 + 4.0 * t32828 + 4.0 * t32830 + 4.0 * t32832 + t33375 + t8564;
    (t33359, t33362, t33363, t33367, t33370, t33374, t33375, t33381)
}
