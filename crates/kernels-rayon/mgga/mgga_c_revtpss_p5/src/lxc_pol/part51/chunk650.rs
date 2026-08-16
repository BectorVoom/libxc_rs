//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 650/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk650(t225: f64, t7048: f64, t1949: f64, t213: f64, t1032: f64, t251: f64, t867: f64) -> (f64, f64, f64, f64) {
    let t7049 = t7048 * t225;
    let t7053 = t213 * t1949;
    let t7056 = t251 * t1032;
    let t7057 = t7056 * t867;
    (t7049, t7053, t7056, t7057)
}
