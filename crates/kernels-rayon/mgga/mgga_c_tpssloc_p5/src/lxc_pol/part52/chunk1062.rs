//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1062/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1062(t12725: f64, t1873: f64, t19456: f64, t4028: f64, t6534: f64, t1458: f64, t649: f64) -> (f64, f64, f64, f64) {
    let t26109 = 2.0_f64 * t12725 * t1873;
    let t26111 = 2.0_f64 * t19456 * t1873;
    let t26113 = 2.0_f64 * t4028 * t6534;
    let t26114 = t649 * t1458;
    (t26109, t26111, t26113, t26114)
}
