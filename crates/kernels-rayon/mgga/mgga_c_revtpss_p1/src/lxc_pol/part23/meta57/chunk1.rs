//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 406/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk406(t1134: f64, t1139: f64, t281: f64, t414: f64, t926: f64, t240: f64, t462: f64) -> (f64, f64, f64, f64) {
    let t1140 = t1139 * t1134;
    let t1143 = t281 * t926 * t414;
    let t1144 = 0.82156666666666666667e-1_f64 * t1143;
    let t1145 = t240 * t462;
    (t1140, t1143, t1144, t1145)
}
