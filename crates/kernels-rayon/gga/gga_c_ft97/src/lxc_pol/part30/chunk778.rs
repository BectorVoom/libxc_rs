//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 778/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk778(t33821: f64, t33822: f64, t684: f64, t33820: f64, t294: f64, t7639: f64, t7242: f64, t7584: f64, t824: f64, t7512: f64, t7638: f64, t1476: f64, t6260: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33824 = t33821 * t33822 * t684;
    let t33825 = t33820 * t33824;
    let t33828 = 1.0_f64 / t7639 / t294;
    let t33829 = t33828 * t7242;
    let t33830 = t7584 * t824;
    let t33831 = t33829 * t33830;
    let t33833 = t7638 * t7512 * t33831;
    let t33835 = t1476 * t6260;
    (t33824, t33825, t33828, t33829, t33830, t33831, t33833, t33835)
}
