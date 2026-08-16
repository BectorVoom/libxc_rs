//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 620/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk620(t678: f64, t7939: f64, t202: f64, t998: f64, t461: f64, t674: f64) -> (f64, f64, f64, f64) {
    let t7940 = t7939 * t678;
    let t7942 = t998 * t202;
    let t7943 = t7942 * t461;
    let t7944 = t7943 * t674;
    (t7940, t7942, t7943, t7944)
}
