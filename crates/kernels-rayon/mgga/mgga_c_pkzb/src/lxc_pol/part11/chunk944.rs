//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 944/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk944(t10101: f64, t10146: f64, t10239: f64, t10280: f64, t158: f64, t3909: f64, t6546: f64, t951: f64, t3254: f64, t3278: f64, t2428: f64, t3928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10282 = t10101 + t10146 + t10239 + t10280;
    let t10283 = t10282 * t158;
    let t10296 = t6546 * t3909;
    let t10297 = t10296 * t951;
    let t10300 = t3254 * t3278;
    let t10305 = t2428 * t3928;
    (t10282, t10283, t10296, t10297, t10300, t10305)
}
