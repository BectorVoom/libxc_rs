//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2435/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2435(t11773: f64, t11865: f64, t11941: f64, t11942: f64, t127: f64, t371: f64, t11937: f64, t11947: f64, t3205: f64, t3206: f64, t676: f64, t11643: f64, t11994: f64) -> (f64, f64, f64, f64, f64) {
    let t42155 = t11865 * t11773;
    let t42170 = t11941 * t371 * t127 * t11942;
    let t42172 = t11947 * t11937;
    let t42176 = t3205 * t371 * t676 * t3206;
    let t42190 = t11994 * t11643;
    (t42155, t42170, t42172, t42176, t42190)
}
