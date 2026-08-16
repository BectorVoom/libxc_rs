//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 637/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk637(t2022: f64, t3: f64, t1401: f64, t1873: f64, t577: f64, t11: f64, t2: f64, t584: f64) -> (f64, f64, f64, f64) {
    let t2023 = t3 * t2022;
    let t2028 = 0.135e2_f64 * t1401 * t1873;
    let t2029 = 0.45e1_f64 * t2022 * t577 + t2028;
    let t2218 = 0.174e1_f64 * t11;
    let t2219 = t2 * t584;
    (t2023, t2029, t2218, t2219)
}
