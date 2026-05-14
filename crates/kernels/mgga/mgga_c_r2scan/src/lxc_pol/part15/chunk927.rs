//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 927/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk927<F: Float>(t1020: F, t3388: F, t1083: F, t1085: F, t1087: F, t1089: F, t11930: F, t2410: F, t3390: F, t3394: F, t3398: F, t3402: F, t3406: F, t3652: F, t3656: F, t3660: F, t3664: F, t839: F) -> (F,) {
    let t11932 = t3388 * t1020;
    let t11960 = -0.64e0 * t11930 - 0.9214113627294e1 * t11932 - 0.9214113627294e1 * t3390 * t1020 - 0.9214113627294e1 * t1083 * t2410 - 0.9214113627294e1 * t3652 * t839 + 0.367387230261e2 * t3394 * t1020 + 0.367387230261e2 * t1085 * t2410 + 0.367387230261e2 * t3656 * t839 - 0.3831420472412e2 * t3398 * t1020 - 0.3831420472412e2 * t1087 * t2410 - 0.3831420472412e2 * t3660 * t839 + 0.1550653405116e2 * t3402 * t1020 + 0.1550653405116e2 * t1089 * t2410 + 0.1550653405116e2 * t3664 * t839 - 0.2177652951264e1 * t3406 * t1020;
    (t11960,)
}
