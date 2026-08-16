//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 638/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk638(t1424: f64, t3972: f64, t729: f64, t762: f64, t1882: f64, t6871: f64, t13839: f64, t6162: f64, t24668: f64, t3859: f64, t14127: f64, t191: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28284 = t1424 * t3972;
    let t28286 = t729 * t762 * t28284;
    let t28289 = t1882 * t6871;
    let t28291 = t13839 * t6162;
    let t28294 = t24668 * t3859;
    let t28295 = t14127 * t28294;
    let t28298 = t191 * t241;
    (t28284, t28286, t28289, t28291, t28294, t28295, t28298)
}
