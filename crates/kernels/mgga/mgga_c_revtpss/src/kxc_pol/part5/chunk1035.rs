//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1035/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1035<F: Float>(t16710: F, t16712: F, t5095: F, t698: F, t3523: F, t5180: F, t1737: F, t3451: F, t1160: F, t5117: F, t3476: F, t16868: F, t16892: F, t16708: F, t1179: F, t5155: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16916 = 4.0 / 9.0 * t16710;
    let t16917 = 2.0 / 9.0 * t16712;
    let t16929 = 0.39862222222222222222e0 * t16710;
    let t16931 = t698 * t5095;
    let t16988 = t5180 * t3523;
    let t17010 = 0.2283111111111111111e-1 * t16710;
    let t17011 = 0.11415555555555555555e-1 * t16712;
    let t17023 = t1737 * t3451;
    let t17026 = t5117 * t1160;
    let t17032 = t1737 * t3476;
    let t17050 = 0.13892666666666666667e0 * t16868;
    let t17052 = 0.34431666666666666666e0 * t16712;
    let t17066 = 0.27785333333333333334e0 * t16892;
    let t17075 = 0.22954444444444444444e0 * t16708;
    let t17089 = t5155 * t1179;
    (t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023, t17026, t17032, t17050, t17052, t17066, t17075, t17089)
}
