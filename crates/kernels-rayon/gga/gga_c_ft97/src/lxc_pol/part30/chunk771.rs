//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 771/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk771(t2567: f64, t7546: f64, t684: f64, t2606: f64, t1882: f64, t7548: f64, t713: f64, t7553: f64, t729: f64, t762: f64, t258: f64, t7440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33759 = t2567 * t7546;
    let t33760 = t33759 * t684;
    let t33761 = t2606 * t33760;
    let t33765 = 2.0_f64 / 9.0_f64 * t1882 * t7548;
    let t33766 = t7553 * t713;
    let t33768 = t729 * t762 * t33766;
    let t33771 = t258 * t7440;
    (t33759, t33760, t33761, t33765, t33766, t33768, t33771)
}
