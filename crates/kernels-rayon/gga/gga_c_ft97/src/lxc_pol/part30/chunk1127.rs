//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1127/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1127(t150709: f64, t35952: f64, t1701: f64, t28633: f64, t2035: f64, t35924: f64, t811: f64, t820: f64, t153047: f64, t4092: f64, t1200: f64, t153116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t153230 = t35952 * t150709;
    let t153241 = t1701 * t28633;
    let t153248 = t2035 * t35924 * t811;
    let t153256 = t2035 * t35924 * t820;
    let t153259 = t4092 * t153047;
    let t153262 = t1200 * t153116;
    (t153230, t153241, t153248, t153256, t153259, t153262)
}
