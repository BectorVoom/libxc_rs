//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 705/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk705(t43: f64, t50: f64, t1891: f64, t47: f64, t6534: f64, t6541: f64, t6713: f64, t6716: f64, t99: f64, t1896: f64, t553: f64, t1900: f64, t52: f64, t6548: f64, t6554: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t6722 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t6713 * t6534 + 4.0_f64 / 3.0_f64 * t6716 * t1891 + 4.0_f64 / 3.0_f64 * t47 * t6541);
    let t6724 = 1.0_f64 / t99 / t50;
    let t6727 = t1896 * t553;
    let t6733 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t6724 * t6548 + 4.0_f64 / 3.0_f64 * t6727 * t1900 + 4.0_f64 / 3.0_f64 * t52 * t6554);
    (t6722, t6724, t6727, t6733)
}
