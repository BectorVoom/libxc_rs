//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1116/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1116(t28683: f64, t28697: f64, t1307: f64, t6207: f64, t6159: f64, t1615: f64, t2109: f64, t27596: f64, t6176: f64, t251: f64, t6193: f64, t1598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28698 = t28683 + t28697;
    let t28700 = t6207 * t1307;
    let t28701 = t6159 * t28700;
    let t28706 = t2109 * t1615;
    let t28707 = t27596 * t28706;
    let t28708 = t6176 * t28707;
    let t28713 = t6193 * t251;
    let t28714 = t28713 * t1598;
    (t28698, t28700, t28701, t28706, t28707, t28708, t28713, t28714)
}
