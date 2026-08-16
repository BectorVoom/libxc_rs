//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 750/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk750(t225: f64, t385: f64, t7810: f64, t1646: f64, t1976: f64, t7145: f64, t1651: f64) -> (f64, f64, f64, f64) {
    let t7812 = t7810 * t225 * t385;
    let t7817 = t1976 * t1646;
    let t7818 = t7145 * t7817;
    let t7821 = t1976 * t1651;
    (t7812, t7817, t7818, t7821)
}
