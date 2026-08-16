//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1036/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1036(t21498: f64, t21529: f64, t21560: f64, t21612: f64, t383: f64, t1625: f64, t5866: f64, t1060: f64, t1615: f64, t1932: f64, t360: f64, t5936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21614 = t21498 + t21529 + t21560 + t21612;
    let t21615 = t383 * t21614;
    let t21617 = t1625 * t5866;
    let t21618 = t21617 * t1060;
    let t21622 = t1932 * t1615 * t360;
    let t21623 = t5936 * t21622;
    (t21614, t21615, t21617, t21618, t21622, t21623)
}
