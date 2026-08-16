//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1421/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1421(t3266: f64, t3307: f64, t3313: f64, t1119: f64, t11269: f64, t3264: f64, t11190: f64, t3316: f64, t11185: f64, t11407: f64, t1117: f64, t3315: f64) -> (f64, f64, f64, f64, f64) {
    let t43994 = 36.0_f64 * t3313 * t3266 * t3307;
    let t43997 = 8.0_f64 * t3264 * t1119 * t11269;
    let t44000 = 0.57895126195293126241e3_f64 * t11190 * t3316 * t3307;
    let t44002 = 0.1929837539843104208e3_f64 * t11185 * t11407;
    let t44006 = 0.64327917994770140268e2_f64 * t3313 * t11269 * t3315 * t1117;
    (t43994, t43997, t44000, t44002, t44006)
}
