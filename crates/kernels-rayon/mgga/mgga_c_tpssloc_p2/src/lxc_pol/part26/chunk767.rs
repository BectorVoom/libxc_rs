//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 767/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk767(t1202: f64, t2140: f64, t1209: f64, t1211: f64, t1207: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t7334 = t1202 * t2140;
    let t7337 = t1209 * sigma2;
    let t7338 = t7337 * t1211;
    let t7339 = t1207 * t7338;
    (t7334, t7337, t7338, t7339)
}
