//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 876/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk876(t2443: f64, t3799: f64, t1103: f64, t228: f64, t231: f64, t625: f64, t2448: f64, t1123: f64, t626: f64, t701: f64, t13625: f64, t13629: f64, t13633: f64, t13636: f64, t13637: f64, t9637: f64, t9645: f64) -> (f64, f64, f64, f64, f64) {
    let t13639 = t3799 * t2443;
    let t13643 = t228 * t1103 * t625 * t231;
    let t13645 = t3799 * t2448;
    let t13647 = t626 * t1123;
    let t13648 = t701 * t13647;
    let t13650 = -0.51074886703703703704e-1_f64 * t13625 - 0.42562405586419753086e-2_f64 * t9645 + 0.2979368391049382716e-1_f64 * t13629 - 0.12768721675925925926e-1_f64 * t13633 - t13636 - 0.17024962234567901235e-1_f64 * t13637 - 0.2269994964609053498e-1_f64 * t13639 + 0.1134997482304526749e-1_f64 * t13643 + t9637 + 0.3404992446913580247e-1_f64 * t13645 - 0.14187468528806584362e-2_f64 * t13648;
    (t13639, t13643, t13645, t13648, t13650)
}
