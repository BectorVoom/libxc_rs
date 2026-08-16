//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1264/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1264(t22573: f64, t8689: f64, t111: f64, t31876: f64, t2174: f64, t7002: f64, t2169: f64, t7020: f64, t1404: f64, t8692: f64, t31949: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t116135 = t8689 * t22573;
    let t116152 = t31876 * t111;
    let t116324 = t7002 * t2174;
    let t116326 = t2169 * t7020;
    let t116328 = t8692 * t1404;
    let t116330 = t576 * t31949;
    (t116135, t116152, t116324, t116326, t116328, t116330)
}
