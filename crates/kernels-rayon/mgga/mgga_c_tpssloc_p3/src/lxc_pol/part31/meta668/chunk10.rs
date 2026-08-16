//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1975/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1975(t225: f64, t29099: f64, t13463: f64, t17057: f64, t17063: f64, t17092: f64, t25168: f64, t26582: f64, t4268: f64, t7087: f64, t7107: f64, t7830: f64, t87042: f64, t87050: f64, t92394: f64, t92486: f64, t98315: f64, t98319: f64, t98322: f64) -> (f64, f64) {
    let t101509 = t29099 * t225;
    let t101540 = 24.0_f64 * t25168 * t92394 * t17063 + 2.0_f64 * t7087 * t17057 - 0.3289868133696452873e-1_f64 * t98315 - 0.3289868133696452873e-1_f64 * t98319 + 0.16449340668482264365e-1_f64 * t98322 - 2.0_f64 * t17092 * t7107 + t92486 - t87042 + 4.0_f64 * t4268 * t26582 + 4.0_f64 * t13463 * t7830 - 0.46058153871750340221e0_f64 * t87050;
    (t101509, t101540)
}
