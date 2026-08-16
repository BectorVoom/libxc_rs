//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2552/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2552(t14134: f64, t3117: f64, t10863: f64, t4571: f64, t13969: f64, t14102: f64, t3039: f64, t10876: f64, t13990: f64, t3048: f64, t14137: f64, t10952: f64, t13970: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49873 = t3117 * t14134;
    let t49877 = t10863 * t4571;
    let t49884 = t3039 * t13969 * t14102;
    let t49887 = t10876 * t13969 * t13990;
    let t49889 = t3048 * t14134;
    let t49892 = t3048 * t14137;
    let t49894 = t10952 * t13970;
    (t49873, t49877, t49884, t49887, t49889, t49892, t49894)
}
