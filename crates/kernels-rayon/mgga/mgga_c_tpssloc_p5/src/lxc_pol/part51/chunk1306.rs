//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1306/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1306(t23097: f64, t25111: f64, t6612: f64, t25115: f64, t6605: f64, t1484: f64, t22690: f64, t23122: f64, t6619: f64, t4162: f64, t8342: f64, t8344: f64) -> (f64, f64, f64, f64) {
    let t118566 = t23097 * t6612 * t25111;
    let t118569 = t6605 * t6612 * t25115;
    let t118573 = t23122 * t22690 * t6619 * t1484;
    let t118576 = t4162 * t8342 * t8344;
    (t118566, t118569, t118573, t118576)
}
