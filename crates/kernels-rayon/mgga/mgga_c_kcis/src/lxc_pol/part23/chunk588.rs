//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 588/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk588(t1464: f64, t5673: f64, t1489: f64, t2001: f64, t1396: f64, t4123: f64, t1948: f64, t4142: f64, t1014: f64, t2007: f64, t1984: f64, t1365: f64, t1930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5674 = t1464 * t5673;
    let t5676 = t2001 * t1489;
    let t5677 = t1396 * t5676;
    let t5678 = t4123 * t5677;
    let t5679 = t1464 * t5678;
    let t5681 = t4142 * t1948;
    let t5684 = t1014 * t2007;
    let t5686 = t1014 * t1984;
    let t5689 = t1930 * t1365;
    (t5674, t5676, t5677, t5678, t5679, t5681, t5684, t5686, t5689)
}
