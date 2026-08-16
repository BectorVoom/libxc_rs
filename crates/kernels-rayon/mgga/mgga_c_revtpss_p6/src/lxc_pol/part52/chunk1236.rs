//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1236/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1236(t128302: f64, t2056: f64, t28042: f64, t94: f64, t34261: f64, t7367: f64, t128289: f64, t128291: f64, t128293: f64, t128294: f64, t128295: f64, t128301: f64, t2007: f64, t25805: f64, t28025: f64, t28050: f64, t28683: f64, t28750: f64, t651: f64, t671: f64, t6985: f64, t7359: f64, t7988: f64) -> f64 {
    let t128303 = t128302 * t2056;
    let t128304 = t94 * t28042;
    let t128305 = t128304 * t2056;
    let t128306 = t34261 * t7367;
    let t128307 = -t2007 * t28683 * t651 - t128291 * t671 - t25805 * t7988 - t28025 * t7988 - t28050 * t7359 - t28750 * t6985 - t128289 - t128293 - t128294 - t128295 - t128301 - t128303 - t128305 - t128306;
    t128307
}
