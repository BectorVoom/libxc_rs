//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3007/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3007(t10696: f64, t1544: f64, t14832: f64, t2394: f64, t2661: f64, t14668: f64, t14923: f64, t124: f64, t4423: f64, t14686: f64, t14931: f64, t4366: f64) -> (f64, f64, f64, f64) {
    let t50396 = t10696 * t1544;
    let t50399 = t2661 * t14832 * t50396 * t2394;
    let t50409 = t14923 * t14668;
    let t50412 = t124 * t4423;
    let t50415 = t14931 * t14686 * t50412 * t4366;
    (t50399, t50409, t50412, t50415)
}
