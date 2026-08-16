//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1126/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1126(t1130: f64, t6613: f64, t6486: f64, t3643: f64, t6835: f64, t1239: f64, t20550: f64, t1281: f64, t20709: f64, t31297: f64, t6301: f64, t19107: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69560 = t1130 * t6613;
    let t70032 = t1130 * t6486;
    let t70071 = t6835 * t3643;
    let t70078 = t20550 * t1239;
    let t70451 = t20709 * t1281;
    let t70767 = t6301 * t31297;
    let t70994 = t19107 * t978;
    (t69560, t70032, t70071, t70078, t70451, t70767, t70994)
}
