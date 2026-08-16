//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2046/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2046(t1035: f64, t1983: f64, t94014: f64, t3057: f64, t7135: f64, t11200: f64, t1976: f64, t3063: f64, t8521: f64, t7143: f64, t36870: f64, t25625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94016 = t1983 * t94014 * t1035;
    let t94023 = t3057 * t7135;
    let t94026 = t11200 * t1976;
    let t94042 = t3063 * t8521;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94068 = t25625 * t7143;
    (t94016, t94023, t94026, t94042, t94053, t94063, t94068)
}
