//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1166/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1166(t10680: f64, t10682: f64, t40310: f64, t10978: f64, t10980: f64, t11568: f64, t122: f64, t2768: f64, t10673: f64, t10675: f64, t10954: f64, t11564: f64, t3446: f64) -> (f64, f64, f64, f64) {
    let t40312 = t10680 * t10682 * t40310;
    let t40315 = t10978 * t10980 * t11568;
    let t40317 = t2768 * t122;
    let t40319 = t10673 * t10675 * t40317;
    let t40331 = t3446 * t10954 * t11564;
    (t40312, t40315, t40319, t40331)
}
