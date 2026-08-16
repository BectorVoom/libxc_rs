//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3468/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3468(t19701: f64, t3127: f64, t3172: f64, t63212: f64, t63214: f64, t63216: f64, t63218: f64, t63220: f64, t63222: f64, t63224: f64, t63226: f64, t63228: f64, t63579: f64, t63581: f64, t63583: f64) -> (f64, f64) {
    let t65376 = t3127 * t3172 * t19701;
    let t65388 = -t63212 + t63214 - t63216 + t63218 + t63220 - t63222 - t63224 + t63226 + t63228 + t63579 + t63581 + t63583;
    (t65376, t65388)
}
