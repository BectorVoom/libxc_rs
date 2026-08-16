//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1390/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1390(t2165: f64, t26135: f64, t652: f64, t120735: f64, t120738: f64, t120740: f64, t120742: f64, t120744: f64, t120747: f64, t120749: f64, t120751: f64, t120753: f64, t1393: f64, t1849: f64, t22461: f64, t31892: f64, t33720: f64, t7989: f64) -> f64 {
    let t123244 = t652 * t2165 * t26135;
    let t123257 = t1393 * t33720 + t1849 * t31892 - 2.0_f64 * t22461 * t7989 - t120735 - t120738 - 2.0_f64 * t120740 - 2.0_f64 * t120742 - 2.0_f64 * t120744 - 2.0_f64 * t120747 - 2.0_f64 * t120749 - 2.0_f64 * t120751 - 2.0_f64 * t120753 - 2.0_f64 * t123244;
    t123257
}
