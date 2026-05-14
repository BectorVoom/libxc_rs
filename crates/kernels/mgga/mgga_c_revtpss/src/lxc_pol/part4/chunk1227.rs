//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1227/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1227<F: Float>(t3667: F, t5362: F, t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F, t480: F, t1803: F, t3650: F, t16708: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12678: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F, F, F) {
    let t17301 = 0.28582678745379824648e-3 * t3667 * t5362;
    let t17303 = t371 * t676 * t1789;
    let t17304 = t1235 * t17303;
    let t17306 = t1769 * t3565;
    let t17307 = t17306 * t225;
    let t17308 = t17307 * t480;
    let t17311 = t3650 * t1803;
    let t17319 = 0.37037037037037037037e-2 * t16708;
    let t17320 = 0.11111111111111111111e-1 * t16710;
    let t17321 = 0.55555555555555555556e-2 * t16712;
    let t17330 = -t12678 + 0.74074074074074074074e-2 * t12297 + 0.18518518518518518519e-2 * t12299 - 0.55555555555555555556e-2 * t12301 - 0.27777777777777777778e-2 * t12303 + 0.37037037037037037037e-2 * t16706 + t17319 - t17320 - t17321 + 0.92592592592592592592e-2 * t16717 - 0.33333333333333333333e-1 * t16722 - 0.11111111111111111111e-1 * t16727 - 0.55555555555555555555e-2 * t16731 + 0.50000000000000000001e-1 * t16735 + 0.33333333333333333334e-1 * t16740 + 0.16666666666666666667e-1 * t16744 + 0.83333333333333333333e-2 * t16748;
    (t17301, t17304, t17306, t17307, t17308, t17311, t17330)
}
