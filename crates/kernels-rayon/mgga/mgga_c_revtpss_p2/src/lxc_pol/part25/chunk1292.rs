//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1292/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1292(t3057: f64, t7135: f64, t11200: f64, t1976: f64, t3063: f64, t8521: f64, t7143: f64, t1035: f64, t1983: f64, t36870: f64, t1096: f64, t19482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94023 = t3057 * t7135;
    let t94026 = t11200 * t1976;
    let t94042 = t3063 * t8521;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94064 = t19482 * t1096;
    (t94023, t94026, t94042, t94053, t94063, t94064)
}
