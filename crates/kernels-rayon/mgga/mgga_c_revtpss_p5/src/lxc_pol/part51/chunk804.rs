//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 804/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk804(t11239: f64, t378: f64, t1035: f64, t7135: f64, t1976: f64, t3046: f64, t994: f64, t11199: f64, t1981: f64) -> (f64, f64, f64, f64, f64) {
    let t25669 = t378 * t11239;
    let t25681 = t1035 * t7135;
    let t25692 = t3046 * t1976;
    let t25695 = t994 * t7135;
    let t25698 = t1981 * t11199;
    (t25669, t25681, t25692, t25695, t25698)
}
