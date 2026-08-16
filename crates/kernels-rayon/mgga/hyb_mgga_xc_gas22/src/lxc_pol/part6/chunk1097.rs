//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1097/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1097(t10703: f64, t839: f64, t848: f64, t10534: f64, t10549: f64, t6528: f64, t6530: f64, t8676: f64, t8721: f64, t251: f64, t10567: f64, t10569: f64, t10572: f64, t10578: f64, t10585: f64, t10587: f64, t6691: f64, t8877: f64) -> (f64, f64, f64, f64) {
    let t10705 = t839 * t10703 * t848;
    let t10718 = -t6528 + 0.23744444444444444444e-1_f64 * t6530 + 0.47488888888888888888e-1_f64 * t8676 - t8721 - 0.17808333333333333333e-1_f64 * t10534 + 0.53425e-1_f64 * t10549;
    let t10720 = 0.621814e-1_f64 * t10718 * t251;
    let t10731 = 0.264729375e1_f64 * t10567 - 0.3529725e1_f64 * t10569 - 0.17648625e1_f64 * t10572 + 0.3529725e1_f64 * t10578 - t6691 + 0.68863333333333333333e0_f64 * t6530 + 0.13772666666666666667e1_f64 * t8676 - t8877 - 0.516475e0_f64 * t10534 + 0.1549425e1_f64 * t10549 - 0.157790625e0_f64 * t10585 + 0.6311625e0_f64 * t10587;
    (t10705, t10718, t10720, t10731)
}
