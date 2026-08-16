//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 910/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk910(t31798: f64, t31805: f64, t31801: f64, t8477: f64, t860: f64, t11007: f64, t822: f64) -> (f64, f64, f64, f64) {
    let t31806 = t31805 * t31798;
    let t31808 = 0.25389723392137995738e-1_f64 * t31806 * t31801;
    let t31809 = t8477 * t860;
    let t31812 = t11007 * t822;
    (t31806, t31808, t31809, t31812)
}
