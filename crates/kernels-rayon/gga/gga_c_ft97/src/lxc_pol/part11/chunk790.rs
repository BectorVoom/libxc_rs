//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 790/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk790(t2842: f64, t863: f64, t2844: f64, t296: f64, t1882: f64, t2751: f64, t869: f64, t309: f64, t875: f64, t2770: f64, t871: f64, t2867: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10688 = t863 * t2842;
    let t10689 = t10688 * t2844;
    let t10690 = t296 * t10689;
    let t10693 = t1882 * t2751;
    let t10695 = t869 * t869;
    let t10696 = 1.0_f64 / t10695;
    let t10697 = t309 * t10696;
    let t10698 = t2844 * t875;
    let t10699 = t10697 * t10698;
    let t10700 = t296 * t10699;
    let t10703 = t2770 * t871;
    let t10704 = t2867 * t684;
    (t10688, t10689, t10690, t10693, t10695, t10696, t10697, t10698, t10699, t10700, t10703, t10704)
}
