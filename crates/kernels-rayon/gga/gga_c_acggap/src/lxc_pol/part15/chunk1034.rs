//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1034/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1034(t2035: f64, t31010: f64, t35246: f64, t30780: f64, t35225: f64, t1439: f64, t1992: f64, t1460: f64, t30148: f64, t7323: f64, t142: f64, t3706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36205 = t2035 * t31010 * t35246;
    let t36207 = t30780 * t35225;
    let t36209 = t1992 * t1439;
    let t36210 = t30780 * t36209;
    let t36213 = t30148 * t1460;
    let t36214 = t2035 * t7323 * t36213;
    let t36222 = t142 * t3706;
    (t36205, t36207, t36209, t36210, t36213, t36214, t36222)
}
