//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1060/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1060(t1235: f64, t12963: f64, t12640: f64, t225: f64, t480: f64, t12621: f64, t482: f64, t371: f64, t372: f64, t12657: f64, t3667: f64, t3678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12964 = t1235 * t12963;
    let t12966 = t12640 * t225;
    let t12967 = t12966 * t480;
    let t12970 = t482 * t12621;
    let t12972 = t371 * t372 * t12970;
    let t12975 = t12657 * t225;
    let t12976 = t12975 * t480;
    let t12979 = t3667 * t3678;
    (t12964, t12966, t12967, t12972, t12975, t12976, t12979)
}
