//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 626/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk626(t28108: f64, t729: f64, t762: f64, t1882: f64, t6923: f64, t1424: f64, t4005: f64, t1131: f64, t6194: f64, t258: f64, t6837: f64, t684: f64) -> (f64, f64, f64, f64, f64) {
    let t28110 = t729 * t762 * t28108;
    let t28113 = t1882 * t6923;
    let t28116 = t729 * t4005 * t1424;
    let t28120 = t729 * t6194 * t1131;
    let t28123 = t258 * t6837;
    let t28124 = t28123 * t684;
    (t28110, t28113, t28116, t28120, t28124)
}
