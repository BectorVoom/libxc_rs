//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 801/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk801(t34053: f64, t871: f64, t1506: f64, t824: f64, t6222: f64, t193: f64, t2862: f64, t319: f64, t33873: f64, t7611: f64, t875: f64, t840: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34054 = t871 * t34053;
    let t34056 = t1506 * t824;
    let t34057 = t6222 * t34056;
    let t34058 = t193 * t34057;
    let t34062 = t2862 * t319 * t33873;
    let t34065 = t7611 * t875;
    let t34067 = t840 * t871 * t34065;
    (t34054, t34056, t34057, t34058, t34062, t34065, t34067)
}
