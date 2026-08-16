//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1775/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1775(t1145: f64, t12287: f64, t141: f64, t12282: f64, t3360: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t12288 = t1145 * t12287;
    let t12289 = t141 * t12288;
    let t12291 = t3360 * t12282;
    let t12292 = t128 * t12291;
    (t12288, t12289, t12291, t12292)
}
