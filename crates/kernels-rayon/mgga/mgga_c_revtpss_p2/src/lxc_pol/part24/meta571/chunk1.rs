//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1750/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1750(t17092: f64, t24212: f64, t16840: f64, t24215: f64, t6534: f64, t1196: f64, t3520: f64, t3523: f64, t6518: f64) -> (f64, f64, f64, f64, f64) {
    let t90349 = 24.0_f64 * t17092 * t24212;
    let t90351 = 0.1929837539843104208e3_f64 * t16840 * t24215;
    let t90352 = t6534 * t6534;
    let t90356 = 0.51947577317044391277e2_f64 * t1196 * t3520 * t90352 * t3523;
    let t90357 = t6518 * t6518;
    (t90349, t90351, t90352, t90356, t90357)
}
