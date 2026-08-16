//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 722/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk722(t6878: f64, t6879: f64, t161: f64, t2024: f64, t127: f64, t136: f64, t2079: f64, t634: f64, t648: f64, t108: f64, t6567: f64, t117: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6880 = t6878 * t6879;
    let t6881 = t161 * t6880;
    let t6884 = t6878 * t2024;
    let t6885 = t161 * t6884;
    let t6888 = t6878 * t127;
    let t6889 = t161 * t6888;
    let t6892 = t2079 * t136;
    let t6893 = t634 * t6892;
    let t6894 = t6893 * t648;
    let t6896 = t108 * t6567;
    let t6899 = 455.0_f64 / 1296.0_f64 * t6896 * t56 * t117;
    (t6880, t6881, t6884, t6885, t6888, t6889, t6892, t6893, t6894, t6896, t6899)
}
