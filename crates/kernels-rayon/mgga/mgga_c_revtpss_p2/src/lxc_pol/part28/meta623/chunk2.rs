//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2209/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2209(t25512: f64, t4820: f64, t25515: f64, t370: f64, t16087: f64, t4890: f64, t93595: f64, t16055: f64, t27493: f64, t15925: f64, t25516: f64, t1087: f64, t93751: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100006 = 0.57165357490759649296e-3_f64 * t25512 * t4820;
    let t100007 = t25515 * t370;
    let t100008 = t16087 * t100007;
    let t100019 = t93595 * t4890;
    let t100024 = 0.11433071498151929859e-2_f64 * t27493 * t16055;
    let t100025 = t15925 * t25516;
    let t100030 = t1087 * t93751;
    (t100006, t100007, t100008, t100019, t100024, t100025, t100030)
}
