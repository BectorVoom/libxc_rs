//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 595/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk595<F: Float>(t3330: F, t6638: F, t143: F, t6432: F, t3399: F, t3400: F, t6272: F, t1154: F, t1646: F, t5153: F, t3410: F, t1155: F, t6276: F, t304: F, t6478: F, t1153: F, t1757: F, t1761: F, t1780: F, t3381: F, t3392: F, t348: F, t365: F, t368: F, t5130: F, t5151: F, t6589: F, t6593: F, t6597: F, t6601: F, t6605: F, t86: F) -> (F, F, F, F, F, F, F, F) {
    let t6640 = 2.0 * t3330 * t6638;
    let t6641 = t6432 * t143;
    let t6661 = t3399 * t3400 * t6272;
    let t6665 = t1154 * t5153 * t1646;
    let t6669 = t1154 * t3410 * t6272;
    let t6673 = t1154 * t1155 * t6276;
    let t6676 = t304 * t6478;
    let t6680 = 0.619125e-2 * t6641 * t348 + 0.1857375e-1 * t1780 * t1757 - 0.123825e-1 * t1780 * t1761 + 0.46434375e-2 * t365 * t6589 - 0.1857375e-1 * t3381 * t6593 + 0.9286875e-2 * t365 * t6597 + 0.123825e-1 * t365 * t6601 - 0.619125e-2 * t365 * t6605 + t3392 - 0.35374814814814814814e-1 * t5130 - 0.53062222222222222222e-1 * t5151 - 0.44218518518518518518e-1 * t1153 * t6661 - 0.53062222222222222222e-1 * t1153 * t6665 + 0.53062222222222222222e-1 * t1153 * t6669 - 0.26531111111111111111e-1 * t1153 * t6673 - 0.39796666666666666666e-1 * t86 * t368 * t6676;
    (t6640, t6641, t6661, t6665, t6669, t6673, t6676, t6680)
}
