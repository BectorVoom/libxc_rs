//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2146/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2146(t25207: f64, t98674: f64, t1940: f64, t1963: f64, t2403: f64, t25198: f64, t25206: f64, t25208: f64, t25449: f64, t27158: f64, t27160: f64, t27169: f64, t27364: f64, t27368: f64, t27395: f64, t4541: f64, t605: f64, t7087: f64, t7783: f64, t98627: f64, t98635: f64, t98637: f64, t98650: f64, t98652: f64, t98659: f64, t98662: f64, t98669: f64) -> f64 {
    let t98675 = t25207 * t98674;
    let t98678 = 3.0_f64 / 2.0_f64 * t2403 * t1963 * t98627 - t98635 - 3.0_f64 * t98637 * t25208 + 3.0_f64 * t4541 * t7783 * t25198 + 3.0_f64 * t2403 * t7087 * t27395 + t98650 - 3.0_f64 / 2.0_f64 * t25206 * t98652 + 3.0_f64 * t2403 * t7087 * t27169 - 3.0_f64 * t25206 * t98659 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t98662 - t1940 * t27368 * t25449 + 6.0_f64 * t98669 * t27160 + t1940 * t27364 * t605 - 6.0_f64 * t27158 * t98675;
    t98678
}
