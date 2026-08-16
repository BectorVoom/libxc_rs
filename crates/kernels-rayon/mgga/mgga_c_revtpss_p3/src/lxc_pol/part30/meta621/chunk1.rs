//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2135/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2135(t27384: f64, t98763: f64, t27375: f64, t890: f64, t27383: f64, t1940: f64, t1963: f64, t2257: f64, t2403: f64, t25206: f64, t25211: f64, t25440: f64, t25445: f64, t27158: f64, t27166: f64, t27364: f64, t27382: f64, t27387: f64, t7010: f64, t7091: f64, t7783: f64, t7787: f64, t92775: f64, t92819: f64, t98733: f64, t98736: f64, t98740: f64, t98743: f64, t98751: f64, t98755: f64, t98760: f64) -> (f64, f64) {
    let t98764 = t98763 * t27384;
    let t98767 = t27375 * t890;
    let t98768 = t27383 * t98767;
    let t98776 = 3.0_f64 * t2403 * t7783 * t25211 + t1940 * t7783 * t2257 / 2.0_f64 - 3.0_f64 * t25206 * t98733 - t1940 * t7091 * t98736 / 2.0_f64 + t1940 * t25445 * t98740 - 3.0_f64 * t25206 * t98743 - t1940 * t25440 * t27387 + 3.0_f64 * t2403 * t27364 * t7010 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t98751 - t1940 * t7091 * t98755 / 2.0_f64 - 3.0_f64 * t27158 * t98760 + 2.0_f64 * t27382 * t98764 + 6.0_f64 * t25206 * t98768 - t1940 * t92775 * t7787 / 2.0_f64 - 3.0_f64 * t92819 * t27166;
    (t98767, t98776)
}
