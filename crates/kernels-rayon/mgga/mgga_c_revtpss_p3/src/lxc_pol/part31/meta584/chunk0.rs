//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2005/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2005(t1096: f64, t4982: f64, t1981: f64, t42058: f64, t7143: f64, t11120: f64, t3140: f64, t1035: f64, t1983: f64, t3057: f64, t7135: f64, t11200: f64, t1976: f64) -> (f64, f64, f64, f64, f64) {
    let t93984 = t4982 * t1096;
    let t93994 = t1981 * t42058 * t7143;
    let t94014 = t3140 * t11120;
    let t94016 = t1983 * t94014 * t1035;
    let t94023 = t3057 * t7135;
    let t94026 = t11200 * t1976;
    (t93984, t93994, t94016, t94023, t94026)
}
