//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1860/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1860(t1516: f64, t81763: f64, t23083: f64, t25094: f64, t1510: f64, t2379: f64, t25119: f64, t815: f64, t2631: f64, t47285: f64, t6605: f64, t9972: f64) -> (f64, f64, f64, f64) {
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87351 = t25119 * t815 * t1510 * t2379;
    let t87355 = t6605 * t9972 * t47285 * t2631;
    (t87345, t87347, t87351, t87355)
}
