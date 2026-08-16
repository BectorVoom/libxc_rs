//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2924/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2924(t77778: f64, t77797: f64, t923: f64, t52035: f64, t52037: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t77539: f64, t77543: f64, t77547: f64) -> (f64, f64, f64) {
    let t77798 = t77778 + t77797;
    let t77799 = t923 * t77798;
    let t77801 = -0.543465e1_f64 * t77539 + 0.181155e1_f64 * t77543 + 0.181155e1_f64 * t77547 - 0.12077e1_f64 * t63338 + 0.40256666666666666666e0_f64 * t63340 + 0.33547222222222222222e0_f64 * t63342 + 0.181155e1_f64 * t63361 - 0.12077e1_f64 * t63371 + 0.80513333333333333336e0_f64 * t52035 - 0.26837777777777777779e0_f64 * t52037 + 0.16504875e0_f64 * t77799;
    (t77798, t77799, t77801)
}
