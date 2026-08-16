//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 905/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk905(t255: f64, t675: f64, t2371: f64, t2492: f64, t3977: f64, t10052: f64, t737: f64, t2372: f64, t754: f64, t10: f64, t16: f64, t2404: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53798 = t675 * t255;
    let t53891 = t2371 * t255;
    let t53923 = t2492 * t3977;
    let t53927 = t737 * t10052;
    let t53942 = t2372 * t754;
    let t54032 = t10 * t16 * t2404;
    (t53798, t53891, t53923, t53927, t53942, t54032)
}
