//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 690/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk690(t22690: f64, t6968: f64, t22642: f64, t1351: f64, t1372: f64, t550: f64, t6976: f64, t1992: f64, t12272: f64, t268: f64, t534: f64, t6559: f64) -> (f64, f64, f64, f64, f64) {
    let t22691 = t22690 * t6968;
    let t22692 = t22642 * t22691;
    let t22693 = 0.82246703342411321824e-2_f64 * t22692;
    let t22694 = t1372 * t1351;
    let t22695 = t22694 * t550;
    let t22696 = t6976 * t22695;
    let t22697 = t1992 * t22696;
    let t22699 = t12272 * t550;
    let t22700 = t6976 * t22699;
    let t22701 = t1992 * t22700;
    let t22704 = t6559 * t534 * t268;
    (t22692, t22693, t22697, t22701, t22704)
}
