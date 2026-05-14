//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 759/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk759<F: Float>(t12048: F, t3110: F, t317: F, t522: F, t323: F, t526: F, t8291: F, t10138: F, t534: F, t333: F, t3754: F, t740: F, t113: F, t11425: F, t11966: F, t518: F) -> (F, F, F, F, F, F, F) {
    let t12049 = 0.62154466893555682512e-3 * t12048;
    let t12058 = 0.27323333333333333333e-1 * t317 * t3110 * t522;
    let t12061 = 0.77488888888888888888e-2 * t323 * t8291 * t526;
    let t12062 = t10138 * t534;
    let t12064 = 0.72818958333333333333e-4 * t333 * t12062;
    let t12065 = t740 * t3754;
    let t12070 = t113 * t11425;
    let t12084 = 0.14055920378328537299e-1 * t11966 * t518;
    (t12049, t12058, t12061, t12064, t12065, t12070, t12084)
}
