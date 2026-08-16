//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1998/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1998(t1078: f64, t1982: f64, t93488: f64, t25604: f64, t25610: f64, t3093: f64, t4975: f64, t3058: f64, t8521: f64, t3143: f64, t7135: f64, t11865: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93490 = t1982 * t93488 * t1078;
    let t93497 = t25610 * t25604;
    let t93498 = t3093 * t4975;
    let t93502 = t3058 * t8521;
    let t93516 = t3143 * t7135;
    let t93543 = t11865 * t25516;
    (t93490, t93497, t93498, t93502, t93516, t93543)
}
