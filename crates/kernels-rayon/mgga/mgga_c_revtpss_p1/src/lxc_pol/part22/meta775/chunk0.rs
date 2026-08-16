//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2863/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2863(t11262: f64, t3600: f64, t3605: f64, t3617: f64, t675: f64, t1261: f64, t247: f64, t3363: f64, t3609: f64, t44169: f64, t1263: f64, t215: f64) -> (f64, f64, f64, f64, f64) {
    let t44675 = t3600 * t11262 * t3605;
    let t44693 = t675 * t3617;
    let t44696 = t1261 * t247 * t44693 * t3363;
    let t44698 = t44169 * t3609;
    let t44701 = t215 * t1263;
    (t44675, t44693, t44696, t44698, t44701)
}
