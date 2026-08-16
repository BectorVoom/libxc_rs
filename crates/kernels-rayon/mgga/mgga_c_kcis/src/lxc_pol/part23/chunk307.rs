//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 307/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk307(t1593: f64, t209: f64, t617: f64, t612: f64, t611: f64, t68: f64) -> (f64, f64, f64) {
    let t1595 = t209 * t1593 * t617;
    let t1597 = t612 * t1595 / 576.0_f64;
    let t1598 = t611 * t68;
    (t1595, t1597, t1598)
}
