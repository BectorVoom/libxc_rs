//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1003/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1003<F: Float>(t37066: F, t12025: F, t12027: F, t12030: F, t12034: F, t12037: F, t12039: F, t12040: F, t12046: F, t12048: F, t12049: F, t12053: F, t12055: F, t12058: F, t12061: F, t12063: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40846 = 22.0 / 9.0 * t37066;
    let t41116 = 45.0 / 32.0 * t12025;
    let t41117 = 5.0 / 8.0 * t12027;
    let t41118 = 5.0 / 8.0 * t12030;
    let t41119 = t12034 / 2.0;
    let t41120 = 5.0 / 8.0 * t12037;
    let t41121 = 2.0 * t12039;
    let t41122 = t12040 / 2.0;
    let t41123 = 3.0 / 2.0 * t12046;
    let t41124 = 2.0 * t12048;
    let t41126 = t12049 / 2.0;
    let t41127 = t12053 / 2.0;
    let t41128 = 2.0 * t12055;
    let t41129 = 3.0 / 2.0 * t12058;
    let t41130 = 5.0 / 8.0 * t12061;
    let t41131 = 2.0 * t12063;
    (t40846, t41116, t41117, t41118, t41119, t41120, t41121, t41122, t41123, t41124, t41126, t41127, t41128, t41129, t41130, t41131)
}
