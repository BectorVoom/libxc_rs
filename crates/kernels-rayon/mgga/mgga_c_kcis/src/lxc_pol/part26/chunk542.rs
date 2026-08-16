//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 542/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk542(t1396: f64, t5676: f64, t4123: f64, t1464: f64, t1948: f64, t4142: f64, t1014: f64, t2007: f64, t1984: f64, t1365: f64, t1930: f64, t1929: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5677 = t1396 * t5676;
    let t5678 = t4123 * t5677;
    let t5679 = t1464 * t5678;
    let t5681 = t4142 * t1948;
    let t5684 = t1014 * t2007;
    let t5686 = t1014 * t1984;
    let t5689 = t1930 * t1365;
    let t5691 = t1929 * t990;
    (t5677, t5678, t5679, t5681, t5684, t5686, t5689, t5691)
}
