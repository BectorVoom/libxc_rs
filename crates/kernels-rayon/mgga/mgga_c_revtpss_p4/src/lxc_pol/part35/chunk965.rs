//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 965/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk965(t23872: f64, t23926: f64, t23988: f64, t24040: f64, t225: f64, t385: f64, t1695: f64, t6350: f64, t11121: f64, t23964: f64, t996: f64, t24031: f64) -> (f64, f64, f64, f64, f64) {
    let t24042 = t23872 + t23926 + t23988 + t24040;
    let t24044 = t24042 * t225 * t385;
    let t24047 = t6350 * t1695;
    let t24048 = t11121 * t24047;
    let t24061 = t996 * t23964;
    let t24068 = t996 * t24031;
    (t24042, t24044, t24048, t24061, t24068)
}
