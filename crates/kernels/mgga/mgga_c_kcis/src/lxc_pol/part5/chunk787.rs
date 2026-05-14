//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 787/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk787<F: Float>(t1830: F, t1220: F, t3557: F, t3564: F, t4612: F, t4706: F, t6328: F, t6332: F, t6336: F, t6341: F, t6343: F, t6375: F, t6377: F, t6381: F, t6384: F, t6387: F) -> (F, F, F) {
    let t6788 = t1830 * t1830;
    let t6789 = t6788 * t1220;
    let t6804 = -0.17648625e1 * t6341 + 0.3529725e1 * t6343 + t3557 + 0.34431666666666666666e0 * t4612 - 0.34431666666666666667e0 * t6328 + 0.103295e1 * t6332 - 0.516475e0 * t6336 + 0.31558125e0 * t6375 + 0.6311625e0 * t6377 + t3564 + 0.13892666666666666667e0 * t4706 - 0.34731666666666666667e-1 * t6381 + 0.20839e0 * t6384 - 0.104195e0 * t6387;
    (t6788, t6789, t6804)
}
