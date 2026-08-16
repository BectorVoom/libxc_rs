//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1812/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1812(t1599: f64, t23588: f64, t23384: f64, t7554: f64, t1065: f64, t7624: f64, t3174: f64, t7614: f64, t986: f64, t6805: f64, t7607: f64, t1949: f64, t4542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25447 = t1599 * t23588;
    let t25450 = t23384 * t7554;
    let t25452 = t7624 * t1065;
    let t25453 = t3174 * t25452;
    let t25456 = t986 * t7614;
    let t25459 = t1599 * t6805;
    let t25465 = t23384 * t7607;
    let t25467 = t4542 * t1949;
    (t25447, t25450, t25453, t25456, t25459, t25465, t25467)
}
