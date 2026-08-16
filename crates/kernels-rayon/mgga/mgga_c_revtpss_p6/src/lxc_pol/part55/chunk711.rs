//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 711/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk711(t1445: f64, t2027: f64, t2103: f64, t213: f64, t561: f64, t7292: f64, t7295: f64, t7495: f64, t7498: f64, t7507: f64, t7511: f64, t7517: f64, t7519: f64, t7523: f64, t7528: f64, t7532: f64) -> f64 {
    let t7535 = -t7495 + t7498 + 0.65854491829355115987e0_f64 * t213 * t7507 * t561 - 0.65854491829355115987e0_f64 * t7511 * t1445 + t7517 - t7519 - 0.4336814094102599731e0_f64 * t7292 * t2103 + 0.8673628188205199462e0_f64 * t7295 * t7523 + 0.4336814094102599731e0_f64 * t7295 * t7528 - 0.4336814094102599731e0_f64 * t2027 * t7532;
    t7535
}
