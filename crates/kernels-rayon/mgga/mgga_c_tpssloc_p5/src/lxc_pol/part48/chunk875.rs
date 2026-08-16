//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 875/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk875(t4034: f64, t8533: f64, t1873: f64, t7156: f64, t652: f64, t1388: f64, t2018: f64, t26558: f64, t26161: f64, t24462: f64, t24465: f64, t7015: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31771 = 2.0_f64 * t4034 * t8533;
    let t31772 = t7156 * t1873;
    let t31774 = 2.0_f64 * t652 * t31772;
    let t31775 = t2018 * t1388;
    let t31776 = t26558 * t31775;
    let t31778 = 2.0_f64 * t26161 * t31776;
    let t31799 = 0.135e2_f64 * t24462 * t1873;
    let t31801 = 27.0_f64 * t24465 * t7015;
    (t31771, t31772, t31774, t31775, t31776, t31778, t31799, t31801)
}
