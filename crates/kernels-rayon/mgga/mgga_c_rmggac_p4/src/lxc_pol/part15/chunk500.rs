//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 500/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk500(t1941: f64, t321: f64, t333: f64, t1743: f64, t338: f64, t352: f64, t4697: f64, t4705: f64, t4997: f64, t4998: f64, t108: f64, t1915: f64) -> (f64, f64, f64, f64, f64) {
    let t6332 = t1941 * t321;
    let t6335 = t1941 * t333;
    let t6338 = t338 * t1743;
    let t6339 = t6338 * t352;
    let t6344 = -t4697 - t4997 + t4998 + t4705;
    let t6349 = t1915 * t108;
    (t6332, t6335, t6339, t6344, t6349)
}
