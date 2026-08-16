//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 283/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk283(t24: f64, t135: f64, t273: f64, t830: f64, t855: f64, t895: f64, t897: f64, t902: f64, t955: f64, t957: f64, t507: f64, t422: f64, t423: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t960 = t135 * t273 * t955 * t957 - t830 + t855 + t895 + t897 - t902;
    let t962 = piecewise3(t90, 0.0_f64, t507);
    let t966 = piecewise3(t332, 0.0_f64, t422 * t962 / 2.0_f64 + t960 * t423 / 2.0_f64);
    (t960, t962, t966)
}
