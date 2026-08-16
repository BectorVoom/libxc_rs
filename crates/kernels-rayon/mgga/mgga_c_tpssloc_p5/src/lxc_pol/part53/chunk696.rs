//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 696/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk696(t6553: f64, t8547: f64, t1880: f64, t1894: f64, t2047: f64, t214: f64, t191: f64, t2079: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8548 = t6553 * t8547;
    let t8549 = t1880 * t8548;
    let t8556 = t1894 * t2047;
    let t8557 = t214 * t8556;
    let t8558 = t1880 * t8557;
    let t8606 = t2079 * t191;
    let t8607 = t8606 * t192;
    (t8548, t8549, t8556, t8557, t8558, t8606, t8607)
}
