//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 404/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk404<F: Float>(t1305: F, t1328: F, t1309: F, t1320: F, t1325: F, t1332: F) -> (F, F, F) {
    let t1566 = 0.516475e0 * t1305;
    let t1569 = 0.104195e0 * t1328;
    let t1571 = 0.3529725e1 * t1320 - t1566 - 0.516475e0 * t1309 + 0.6311625e0 * t1325 - t1569 - 0.104195e0 * t1332;
    (t1566, t1569, t1571)
}
