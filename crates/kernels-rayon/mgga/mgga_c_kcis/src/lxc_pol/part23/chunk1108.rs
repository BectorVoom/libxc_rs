//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1108/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1108(t1548: f64, t5748: f64, t27520: f64, t6029: f64, t1552: f64, t5752: f64, t5932: f64, t7948: f64, t2066: f64, t3738: f64, t1928: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28600 = t5748 * t1548;
    let t28602 = t27520 * t6029;
    let t28604 = t5752 * t1552;
    let t28606 = t7948 * t5932;
    let t28608 = t3738 * t2066;
    let t28610 = t570 * t1928;
    (t28600, t28602, t28604, t28606, t28608, t28610)
}
