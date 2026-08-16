//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 280/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk280(t3842: f64, t729: f64, t762: f64, t1091: f64, t724: f64, t773: f64, t265: f64, t3746: f64, t1175: f64, t684: f64, t1168: f64, t713: f64) -> (f64, f64, f64, f64, f64) {
    let t3844 = t729 * t762 * t3842;
    let t3848 = t724 * t773 * t1091;
    let t3852 = t724 * t265 * t3746;
    let t3856 = t724 * t1175 * t684;
    let t3859 = t1168 * t713;
    (t3844, t3848, t3852, t3856, t3859)
}
