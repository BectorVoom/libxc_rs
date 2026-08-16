//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3009/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3009(t14786: f64, t231: f64, t40834: f64, t854: f64, t14833: f64, t236: f64, t2453: f64, t9794: f64, t125: f64, t14662: f64, t10777: f64, t14671: f64, t14917: f64, t40725: f64) -> (f64, f64, f64, f64, f64) {
    let t50451 = t14786 * t231;
    let t50453 = t40834 * t854 * t50451;
    let t50457 = t2453 * t236 * t9794 * t14833;
    let t50459 = t125 * t14662;
    let t50466 = t10777 * t40725 * t14671 * t14917;
    (t50451, t50453, t50457, t50459, t50466)
}
