//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 805/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk805(t1130: f64, t1767: f64, t1773: f64, t3217: f64, t1697: f64, t2835: f64, t1141: f64, t5034: f64, t1778: f64, t3329: f64) -> (f64, f64, f64, f64, f64) {
    let t14628 = t1130 * t1767;
    let t14649 = t3217 * t1773;
    let t14654 = t1697 * t2835;
    let t14665 = t5034 * t1141;
    let t14668 = t1778 * t3329;
    (t14628, t14649, t14654, t14665, t14668)
}
