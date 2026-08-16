//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2080/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2080(t90645: f64, t22716: f64, t7701: f64, t1834: f64, t212: f64, t22642: f64, t6890: f64, t26215: f64, t81228: f64, t81326: f64, t2015: f64, t40590: f64) -> (f64, f64, f64, f64, f64) {
    let t90646 = 0.82246703342411321824e-2_f64 * t90645;
    let t90659 = t22716 * t7701;
    let t90663 = t22642 * t212 * t1834 * t6890;
    let t90686 = t81228 * t81326 * t26215;
    let t90687 = 0.16449340668482264365e-1_f64 * t90686;
    let t90696 = t40590 * t2015;
    (t90646, t90659, t90663, t90687, t90696)
}
