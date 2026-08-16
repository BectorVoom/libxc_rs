//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2569/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2569(t1667: f64, t9709: f64, t14712: f64, t699: f64, t2403: f64, t4778: f64, t14750: f64, t690: f64) -> (f64, f64, f64, f64) {
    let t50846 = t9709 * t1667;
    let t50848 = t699 * t14712;
    let t50853 = t2403 * t4778;
    let t50903 = t690 * t14750;
    (t50846, t50848, t50853, t50903)
}
