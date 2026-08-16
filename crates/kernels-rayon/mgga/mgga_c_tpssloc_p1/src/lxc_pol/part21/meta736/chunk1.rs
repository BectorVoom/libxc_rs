//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2597/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2597(t11697: f64, t15473: f64, t3577: f64, t11698: f64, t15740: f64, t10401: f64, t15567: f64, t3610: f64, t11692: f64, t15563: f64, t15743: f64, t3490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52619 = t3577 * t11697 * t15473;
    let t52621 = t15740 * t11698;
    let t52627 = t15567 * t10401;
    let t52628 = t3610 * t52627;
    let t52649 = t11692 * t11697 * t15563;
    let t52653 = t3490 * t15743;
    (t52619, t52621, t52627, t52628, t52649, t52653)
}
