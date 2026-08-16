//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 715/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk715(t1867: f64, t6405: f64, t6407: f64, t601: f64, t1: f64, t1906: f64, t598: f64, t1864: f64, t586: f64, t6347: f64, t1847: f64, t1859: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6814 = t6405 * t6407 * t1867;
    let t6816 = 0.1038945353962551798e3_f64 * t601 * t6814;
    let t6817 = t1906 * t1;
    let t6818 = t6817 * t598;
    let t6819 = 0.54934665110259479823e-3_f64 * t6818;
    let t6820 = t1864 * t586;
    let t6821 = t6820 * t6347;
    let t6823 = 0.51947267698127589897e2_f64 * t601 * t6821;
    let t6825 = t1847 * t1859 * t588;
    (t6814, t6816, t6817, t6819, t6820, t6821, t6823, t6825)
}
