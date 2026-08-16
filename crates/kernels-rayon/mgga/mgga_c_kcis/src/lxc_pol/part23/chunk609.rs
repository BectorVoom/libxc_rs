//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 609/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk609(t4135: f64, t5875: f64, t1395: f64, t1464: f64, t1497: f64, t2001: f64) -> (f64, f64, f64, f64) {
    let t5876 = t4135 * t5875;
    let t5877 = t1395 * t5876;
    let t5878 = t1464 * t5877;
    let t5880 = t2001 * t1497;
    (t5876, t5877, t5878, t5880)
}
