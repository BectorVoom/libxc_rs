//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 256/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk256(t774: f64, t803: f64, t781: f64, t792: f64, t797: f64, t807: f64) -> (f64, f64, f64) {
    let t823 = 0.516475e0_f64 * t774;
    let t826 = 0.104195e0_f64 * t803;
    let t828 = 0.3529725e1_f64 * t792 - t823 + 0.1549425e1_f64 * t781 + 0.6311625e0_f64 * t797 - t826 + 0.312585e0_f64 * t807;
    (t823, t826, t828)
}
