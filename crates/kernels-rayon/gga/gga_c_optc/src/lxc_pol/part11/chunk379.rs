//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 379/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk379(t1849: f64, t587: f64, t1767: f64, t1770: f64, t1773: f64, t1777: f64, t1779: f64, t1782: f64) -> (f64, f64) {
    let t1850 = t1849 * t587;
    let t1859 = -0.57538888888888888889e0_f64 * t1767 + 0.11507777777777777778e1_f64 * t1770 + 0.40256666666666666667e0_f64 * t1773 + 0.366775e-1_f64 * t1777 + 0.73355e-1_f64 * t1779 + 0.137975e0_f64 * t1782;
    (t1850, t1859)
}
