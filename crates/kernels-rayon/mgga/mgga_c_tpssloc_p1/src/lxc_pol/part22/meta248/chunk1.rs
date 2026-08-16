//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1360/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1360(t10294: f64, t10544: f64, t2884: f64, t922: f64, t302: f64, t2887: f64, t310: f64, t2791: f64, t888: f64, t2929: f64, t938: f64, t10523: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10784 = 0.46308888888888888888e0_f64 * t10294;
    let t10785 = 0.16068111111111111111e1_f64 * t10544;
    let t10810 = 1.0_f64 / t2884 / t922;
    let t10811 = t302 * t10810;
    let t10813 = 1.0_f64 / t2887 / t310;
    let t10817 = t888 * t2791;
    let t10825 = t938 * t2929;
    let t10828 = t315 * t10523;
    (t10784, t10785, t10810, t10811, t10813, t10817, t10825, t10828)
}
