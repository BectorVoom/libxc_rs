//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1004/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1004(t2035: f64, t34399: f64, t7935: f64, t8764: f64, t13272: f64, t8736: f64, t7937: f64, t2163: f64, t7741: f64, t651: f64, t7586: f64, t7742: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34400 = t34399 * t2035;
    let t34401 = t8764 * t7935;
    let t34402 = t13272 * t8736;
    let t34424 = t8764 * t7937;
    let t34428 = t2163 * t7741;
    let t34429 = t651 * t34428;
    let t34434 = t7586 * t7742;
    (t34400, t34401, t34402, t34424, t34428, t34429, t34434)
}
