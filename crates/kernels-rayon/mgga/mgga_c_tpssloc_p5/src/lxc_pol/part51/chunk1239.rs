//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1239/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1239(t33518: f64, t33552: f64, t113: f64, t7756: f64, t8607: f64, t1442: f64, t8595: f64, t1976: f64, t32674: f64, t32676: f64, t32679: f64, t33360: f64, t33361: f64, t33364: f64, t33365: f64, t33367: f64, t7787: f64, t7941: f64, t8329: f64, t8450: f64) -> (f64, f64) {
    let t33553 = t33518 + t33552;
    let t33554 = t113 * t33553;
    let t33555 = t8607 * t7756;
    let t33556 = t1442 * t8595;
    let t33558 = -t1976 * t7787 + t7941 * t8450 - t32674 - t32676 - t32679 - t33360 - t33361 + t33364 + t33365 - t33367 - t33554 - t33555 - t33556 - t8329;
    (t33553, t33558)
}
