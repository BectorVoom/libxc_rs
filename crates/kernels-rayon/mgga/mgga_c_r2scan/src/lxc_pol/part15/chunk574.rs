//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 574/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk574(t11: f64, t1643: f64, t1645: f64, t2736: f64, t2804: f64, t2819: f64, t2828: f64, t5: f64, param_eta: f64) -> f64 {
    let t2832 = t1643 - 5.0_f64 / 3.0_f64 * t1645 - 5.0_f64 / 3.0_f64 * t2736 + 5.0_f64 * t5 * t11 * t2804 - 45.0_f64 * param_eta * (t2819 + t2828);
    t2832
}
