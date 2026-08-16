//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2103/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2103(t22267: f64, t4018: f64, t22079: f64, t5673: f64, t5675: f64, t1353: f64, t6836: f64, t828: f64, t9942: f64, t1868: f64, t5591: f64, t4012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22268 = t4018 * t22267;
    let t22271 = t5673 * t22079 * t5675;
    let t22274 = t6836 * t1353;
    let t22276 = t9942 * t828 * t22274;
    let t22279 = t1868 * t5591;
    let t22281 = t4012 * t828 * t22279;
    (t22268, t22271, t22274, t22276, t22279, t22281)
}
