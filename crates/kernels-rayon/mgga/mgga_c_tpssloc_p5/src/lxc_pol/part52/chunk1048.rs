//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1048/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1048(t265: f64, t394: f64, t1068: f64, t1070: f64, t1637: f64, t193: f64, t23738: f64, t23742: f64, t25836: f64, t25840: f64, t25845: f64, t25882: f64, t336: f64, t4696: f64, t4700: f64, t6822: f64) -> f64 {
    let t395 = t265 < t394;
    let t25883 = piecewise3(t395, t1070 * t193 * t25836 * t336 - t1068 * t25840 * t4700 - t1637 * t23738 * t4700 + 2.0_f64 * t23742 * t25845 * t4700 - t4696 * t4700 * t6822, t25882);
    t25883
}
