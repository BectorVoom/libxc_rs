//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 641/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk641(t242: f64, t27989: f64, t684: f64, t6947: f64, t724: f64, t3859: f64, t6154: f64, t729: f64, t1882: f64, t6932: f64, t6930: f64, t14175: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28326 = t242 * t27989;
    let t28330 = t724 * t6947 * t684;
    let t28334 = t729 * t6154 * t3859;
    let t28338 = t1882 * t6932;
    let t28340 = t6930 * t684;
    let t28341 = t14175 * t28340;
    (t28326, t28330, t28334, t28338, t28340, t28341)
}
