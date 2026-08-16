//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1571/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1571(t23083: f64, t6606: f64, t1891: f64, t22822: f64, t133: f64, t6601: f64, t6590: f64, t6604: f64) -> (f64, f64, f64, f64) {
    let t23084 = t23083 * t6606;
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23097 = t6590 * t6604;
    (t23084, t23093, t23095, t23097)
}
