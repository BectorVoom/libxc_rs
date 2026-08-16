//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 324/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk324(t344: f64, t883: f64, t607: f64, t977: f64, t906: f64, t910: f64) -> (f64, f64, f64, f64) {
    let t978 = t344 * t883;
    let t979 = t978 * t607;
    let t980 = t977 * t979;
    let t984 = t906 / 6.0_f64 + t910 / 6.0_f64;
    (t978, t979, t980, t984)
}
