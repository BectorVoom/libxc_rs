//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2415/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2415(t1637: f64, t17198: f64, t4696: f64, t4700: f64, t60867: f64, t68905: f64, t68910: f64, t68912: f64, t68916: f64, t68918: f64, t68920: f64, t68923: f64, t68926: f64, t68930: f64) -> f64 {
    let t68931 = -3.0_f64 * t1637 * t4700 * t60867 + 6.0_f64 * t17198 * t4696 * t4700 - t68905 + t68910 - t68912 + t68916 + t68918 - t68920 - t68923 + t68926 - t68930;
    t68931
}
