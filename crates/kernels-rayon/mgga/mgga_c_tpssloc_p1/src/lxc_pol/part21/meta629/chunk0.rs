//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2410/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2410(t10108: f64, t257: f64, t68: f64, t2627: f64, t2710: f64, t233: f64, t9970: f64, t2632: f64, t2678: f64, t9975: f64, t2696: f64, t9612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40889 = 1.0_f64 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40895 = t2627 * t2710;
    let t40931 = 1.0_f64 / t9970 / t233;
    let t40933 = t2632 * t2632;
    let t40951 = t9975 * t2678;
    let t40961 = t9612 * t2696;
    (t40890, t40895, t40931, t40933, t40951, t40961)
}
