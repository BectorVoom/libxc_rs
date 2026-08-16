//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1186/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1186(t112741: f64, t112743: f64, t113053: f64, t118672: f64, t118729: f64, t118768: f64, t13053: f64, t13065: f64, t1528: f64, t25348: f64, t2597: f64, t2713: f64, t2718: f64, t30630: f64, t30647: f64, t30729: f64, t32796: f64, t4147: f64, t4268: f64, t6632: f64, t6662: f64, t6663: f64, t7537: f64, t8353: f64, t855: f64, t858: f64) -> f64 {
    let t118791 = 0.82246703342411321825e-2_f64 * t112741;
    let t118792 = 0.76763589786250567036e-1_f64 * t112743;
    let t118793 = -t113053 * t1528 - t118672 + 4.0_f64 * t855 * t2718 * t6662 * t7537 - t855 * t858 * (t118729 + t118768) + 4.0_f64 * t4147 * t30630 - 6.0_f64 * t2713 * t32796 - 2.0_f64 * t25348 * t6663 - t4268 * t30729 + 4.0_f64 * t25348 * t6632 - 6.0_f64 * t2597 * t32796 + 2.0_f64 * t4147 * t30647 + 2.0_f64 * t13053 * t8353 + 2.0_f64 * t13065 * t8353 + 4.0_f64 * t4268 * t30630 + t118791 + t118792;
    t118793
}
