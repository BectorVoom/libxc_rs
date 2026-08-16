//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1205/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1205(t2693: f64, t5576: f64, t2627: f64, t5631: f64, t10143: f64, t5660: f64, t2394: f64, t5678: f64) -> (f64, f64, f64, f64) {
    let t59288 = t5576 * t2693;
    let t59355 = t2627 * t5631;
    let t59564 = t5660 * t10143;
    let t59657 = t2394 * t5678;
    (t59288, t59355, t59564, t59657)
}
