//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3396/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3396(t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52065: f64, t63393: f64, t63396: f64, t63399: f64, t63469: f64, t63471: f64) -> f64 {
    let t63764 = 0.10735111111111111112e1_f64 * t52035 - 0.35783703703703703705e0_f64 * t52037 - 0.80513333333333333336e0_f64 * t52039 - 0.40256666666666666668e0_f64 * t52041 - 0.80513333333333333335e0_f64 * t52045 + 0.26837777777777777778e0_f64 * t52047 + 0.13418888888888888889e0_f64 * t52049 + 0.22364814814814814815e0_f64 * t52051 + 0.11038e0_f64 * t52065 - 0.14717333333333333333e0_f64 * t63393 + 0.16504875e0_f64 * t63396 - 0.72462e1_f64 * t63399 + 0.16504875e0_f64 * t63469 + 0.19419375e1_f64 * t63471;
    t63764
}
