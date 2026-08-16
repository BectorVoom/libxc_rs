//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 642/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk642(t448: f64, t4752: f64, t1306: f64, t1645: f64, t1616: f64, t596: f64, t1328: f64, t165: f64, t1559: f64, t19: f64) -> (f64, f64, f64, f64, f64) {
    let t4753 = t4752 * t448;
    let t4762 = t1645 * t1306;
    let t4771 = t1616 * t596;
    let t4774 = t165 * t1328;
    let t4779 = t1559 * t19;
    (t4753, t4762, t4771, t4774, t4779)
}
