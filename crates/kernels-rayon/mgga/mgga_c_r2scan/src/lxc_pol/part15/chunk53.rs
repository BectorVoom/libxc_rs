//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 53/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk53(t128: f64, t20: f64, t4: f64, t108: f64, t6: f64, t114: f64) -> (f64, f64, f64, f64) {
    let t129 = t128 * t20;
    let t130 = 1.0_f64 / t4;
    let t132 = t6 * t108;
    let t133 = 1.0_f64 / t114;
    (t129, t130, t132, t133)
}
