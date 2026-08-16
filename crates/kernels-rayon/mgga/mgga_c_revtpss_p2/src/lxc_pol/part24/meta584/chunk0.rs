//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1816/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1816(t48269: f64, t85912: f64, t73481: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t47014: f64, t47017: f64, t47020: f64, t47059: f64) -> (f64, f64, f64, f64) {
    let t91968 = 0.20779030926817756511e3_f64 * t48269;
    let t91969 = 0.73245789224026180216e-3_f64 * t85912;
    let t91970 = 0.35089341735807877242e1_f64 * t73481;
    let t91971 = -t39786 - t39791 - t39795 - t47014 - t91968 - t91969 + t47017 + t47020 + t39799 + t47059 + t39807 - t39813 - t91970;
    (t91968, t91969, t91970, t91971)
}
