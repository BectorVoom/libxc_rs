//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2602/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2602(t11007: f64, t252: f64, t2782: f64, t6048: f64, t886: f64, t14481: f64, t1569: f64, t18805: f64, t41066: f64, t10995: f64, t122: f64, t18796: f64, t2466: f64) -> (f64, f64, f64, f64) {
    let t61419 = t2782 * t252 * t11007 * t6048 * t886;
    let t61422 = t2782 * t1569 * t14481;
    let t61430 = t41066 * t18805;
    let t61437 = t10995 * t18796 * t122 * t2466;
    (t61419, t61422, t61430, t61437)
}
