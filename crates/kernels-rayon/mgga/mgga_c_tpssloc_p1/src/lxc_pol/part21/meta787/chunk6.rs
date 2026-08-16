//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2744/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2744(t16583: f64, t16589: f64, t39249: f64, t39256: f64, t40626: f64, t46341: f64, t57877: f64, t57879: f64, t57880: f64, t57885: f64, t57886: f64, t57888: f64, t57889: f64) -> f64 {
    let t57890 = 24.0_f64 * t16583 * t46341 + 12.0_f64 * t16589 * t46341 - t39249 - t39256 + t40626 + t57877 + t57879 - t57880 - t57885 - t57886 + t57888 - t57889;
    t57890
}
