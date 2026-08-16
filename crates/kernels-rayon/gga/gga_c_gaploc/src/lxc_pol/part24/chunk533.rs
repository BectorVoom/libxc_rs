//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 533/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk533(t448: f64, t999: f64, t535: f64, t988: f64, t2274: f64, t2278: f64, t2283: f64, t2285: f64, t471: f64, t64: f64, t984: f64) -> (f64, f64, f64, f64) {
    let t2738 = t999 * t448;
    let t2741 = t535 * t988;
    let t2748 = -21.0_f64 / 128.0_f64 * t2274 + 21.0_f64 / 4096.0_f64 * t2278 - 7.0_f64 / 4096.0_f64 * t2283 + 7.0_f64 / 128.0_f64 * t2285;
    let t2754 = t2748 * t471 - 4.0_f64 / 3.0_f64 * t984 * t64 - 7.0_f64 / 128.0_f64 * t2274 + 7.0_f64 / 384.0_f64 * t2285;
    (t2738, t2741, t2748, t2754)
}
