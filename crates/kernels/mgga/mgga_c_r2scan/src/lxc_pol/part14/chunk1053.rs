//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1053/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1053<F: Float>(t12039: F, t12040: F, t12046: F, t12048: F, t11185: F, t11188: F, t11192: F, t11193: F, t11195: F, t12043: F, t41116: F, t41117: F, t41118: F, t41119: F, t41120: F, t12049: F) -> (F, F) {
    let t41121 = 2.0 * t12039;
    let t41122 = t12040 / 2.0;
    let t41123 = 3.0 / 2.0 * t12046;
    let t41124 = 2.0 * t12048;
    let t41125 = -t11185 + t41116 - t41117 + t41118 - t41119 + t11188 - t41120 + t11192 + t41121 + t41122 + t12043 - t41123 + t11193 + t11195 + t41124;
    let t41126 = t12049 / 2.0;
    (t41125, t41126)
}
