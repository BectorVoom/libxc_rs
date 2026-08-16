//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1181/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1181(t1592: f64, t42891: f64, t973: f64, t10471: f64, t47840: f64, t10479: f64, t10375: f64, t1612: f64, t1041: f64, t1539: f64, t248: f64, t42749: f64) -> (f64, f64, f64, f64, f64) {
    let t48397 = t973 * t42891 * t1592;
    let t48569 = t47840 * t10471;
    let t48570 = t48569 * t10479;
    let t48670 = t1612 * t10375;
    let t48674 = t1041 * t248 * t42749 * t1539;
    (t48397, t48569, t48570, t48670, t48674)
}
