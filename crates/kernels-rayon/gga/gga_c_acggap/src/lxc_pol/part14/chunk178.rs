//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 178/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk178(t150: f64, t545: f64, t187: f64, t456: f64, t525: f64, t182: f64, t119: f64, t151: f64, t451: f64, t455: f64) -> (f64, f64, f64, f64) {
    let t546 = t545 * t150;
    let t547 = t546 * t187;
    let t550 = t456 * t525;
    let t553 = t182 * t545;
    let t556 = t451 - t455 - 0.65854491829355115987e0_f64 * t151 * t550 + 0.65854491829355115987e0_f64 * t119 * t553;
    (t547, t550, t553, t556)
}
