//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3230/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3230(t1300: f64, t198: f64, t336: f64, t81646: f64, t81649: f64, t81653: f64, t81656: f64, t81660: f64, t82119: f64, t82169: f64, t82220: f64, t82266: f64, t82391: f64, t82394: f64, t82396: f64, t82398: f64, t84241: f64, t84290: f64, t84337: f64, t84947: f64, t84992: f64) -> f64 {
    let t84999 = -t81646 - t81649 + t81653 + t81656 + t81660 + t82119 + t198 * t336 * (t82169 + t82220 + t82266 + t84241 + t84290 + t84337 + t84947 + t84992) * t1300 - t82391 - t82394 - t82396 - t82398;
    t84999
}
