//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2595/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2595(t1227: f64, t49850: f64, t4988: f64, t15568: f64, t3604: f64, t11697: f64, t15473: f64, t3577: f64, t11698: f64, t15740: f64, t10401: f64, t15567: f64) -> (f64, f64, f64, f64, f64) {
    let t52609 = t1227 * t49850 * t4988;
    let t52610 = 5.0_f64 / 20736.0_f64 * t52609;
    let t52615 = t3604 * t15568;
    let t52619 = t3577 * t11697 * t15473;
    let t52621 = t15740 * t11698;
    let t52627 = t15567 * t10401;
    (t52610, t52615, t52619, t52621, t52627)
}
