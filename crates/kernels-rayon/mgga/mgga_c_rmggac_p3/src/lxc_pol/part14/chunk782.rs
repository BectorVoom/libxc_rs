//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 782/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk782(t2186: f64, t7421: f64, t1986: f64, t1995: f64, t305: f64, t321: f64, t2134: f64, t27: f64, t3118: f64, t36271: f64, t7204: f64, t36277: f64, t7192: f64) -> (f64, f64, f64, f64, f64) {
    let t36383 = t2186 * t7421;
    let t36391 = t1986 * t305 * t1995 * t321;
    let t36402 = t2134 * t27 * t3118 * t321;
    let t36416 = t7204 * t36271;
    let t36418 = t7192 * t36277;
    (t36383, t36391, t36402, t36416, t36418)
}
