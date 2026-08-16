//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 425/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk425(t1608: f64, t1610: f64, t286: f64, t1597: f64, t1599: f64, t1603: f64, t622: f64) -> (f64, f64, f64, f64) {
    let t1611 = t1608 * t1610;
    let t1612 = t286 * t1611;
    let t1615 = t1597 + t1599 * t1603 / 576.0_f64 - t1599 * t1612 / 192.0_f64;
    let t1616 = 1.0_f64 / t622;
    (t1611, t1612, t1615, t1616)
}
