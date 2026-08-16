//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 728/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk728(t1920: f64, t25529: f64, t23384: f64, t7604: f64, t4640: f64, t6754: f64, t1611: f64, t6764: f64, t4603: f64, t6717: f64, t4571: f64, t6765: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25530 = t1920 * t25529;
    let t25563 = t23384 * t7604;
    let t25577 = t4640 * t6754;
    let t25580 = t1611 * t6764;
    let t25598 = t6717 * t4603;
    let t25616 = t6765 * t4571;
    (t25530, t25563, t25577, t25580, t25598, t25616)
}
