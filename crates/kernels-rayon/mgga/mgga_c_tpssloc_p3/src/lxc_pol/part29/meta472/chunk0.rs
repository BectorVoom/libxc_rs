//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1808/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1808(t1873: f64, t3652: f64, t652: f64, t6876: f64, t7000: f64, t6880: f64, t9348: f64, t12734: f64, t2314: f64, t6534: f64, t12739: f64, t5113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23831 = t3652 * t1873;
    let t23833 = 2.0_f64 * t652 * t23831;
    let t23835 = 2.0_f64 * t6876 * t7000;
    let t23837 = 6.0_f64 * t6876 * t6880;
    let t23844 = 2.0_f64 * t9348 * t1873;
    let t23846 = 4.0_f64 * t12734 * t1873;
    let t23848 = 4.0_f64 * t2314 * t6534;
    let t23850 = 2.0_f64 * t12739 * t1873;
    let t23852 = 4.0_f64 * t5113 * t6534;
    (t23831, t23833, t23835, t23837, t23844, t23846, t23848, t23850, t23852)
}
