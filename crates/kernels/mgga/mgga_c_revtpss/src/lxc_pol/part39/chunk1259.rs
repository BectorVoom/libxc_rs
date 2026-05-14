//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1259/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1259<F: Float>(t1737: F, t3476: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F, t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F) -> (F, F, F, F) {
    let t17032 = t1737 * t3476;
    let t17050 = 0.13892666666666666667e0 * t16868;
    let t17052 = 0.34431666666666666666e0 * t16712;
    let t17061 = -t17050 + 0.104195e0 * t16871 - t17052 + 0.516475e0 * t16748 + 0.22954444444444444444e0 * t16706 + 0.11577222222222222222e0 * t16876 + 0.11477222222222222222e0 * t12299 + 0.45908888888888888888e0 * t12297 - 0.34431666666666666666e0 * t12301 - 0.17215833333333333333e0 * t12303 - 0.68863333333333333334e0 * t16727;
    let t17066 = 0.27785333333333333334e0 * t16892;
    let t17075 = 0.22954444444444444444e0 * t16708;
    let t17083 = 0.46308888888888888889e-1 * t16908 + 0.6311625e0 * t16927 - 0.68863333333333333333e0 * t16710 + t17075 + 0.46308888888888888889e-1 * t16931 + 0.3529725e1 * t16933 - 0.20659e1 * t16722 + 0.20659e1 * t16740 + 0.103295e1 * t16744 + 0.309885e1 * t16735 + 0.57386111111111111112e0 * t16717;
    (t17032, t17061, t17066, t17083)
}
