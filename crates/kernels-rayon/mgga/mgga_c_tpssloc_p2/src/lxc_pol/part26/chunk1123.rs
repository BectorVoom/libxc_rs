//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1123/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1123(t1891: f64, t22822: f64, t133: f64, t6601: f64, t6590: f64, t6604: f64) -> (f64, f64, f64) {
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23096 = 0.52708876011794399171e-3_f64 * t23095;
    let t23097 = t6590 * t6604;
    (t23093, t23096, t23097)
}
