//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 803/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk803(t24886: f64, t6361: f64, t1501: f64, t6386: f64, t2843: f64, t296: f64, t1508: f64, t2862: f64, t6278: f64, t684: f64, t7686: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34086 = t24886 * t6361;
    let t34089 = t1501 * t6386;
    let t34090 = t2843 * t34089;
    let t34091 = t296 * t34090;
    let t34095 = t2862 * t1508 * t6278;
    let t34099 = t835 * t7686 * t684;
    (t34086, t34089, t34090, t34091, t34095, t34099)
}
