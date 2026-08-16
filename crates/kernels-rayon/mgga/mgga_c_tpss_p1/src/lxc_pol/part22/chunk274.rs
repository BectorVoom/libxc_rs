//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 274/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk274(t865: f64, t866: f64, t846: f64, t833: f64, t839: f64, t301: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t867 = t865 * t866;
    let t869 = 1.0_f64 * t846 * t867;
    let t870 = 0.17123333333333333333e-1_f64 * t833;
    let t872 = -t870 - 0.17123333333333333333e-1_f64 * t839;
    let t875 = t301 * t301;
    let t876 = 1.0_f64 / t875;
    (t867, t869, t870, t872, t875, t876)
}
