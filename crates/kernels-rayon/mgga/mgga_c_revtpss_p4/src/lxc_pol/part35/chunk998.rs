//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 998/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk998(t24493: f64, t3523: f64, t1196: f64, t1179: f64, t1188: f64, t24407: f64, t1832: f64, t6752: f64, t1828: f64, t3737: f64, t6744: f64, t1774: f64) -> (f64, f64, f64, f64, f64) {
    let t24494 = t24493 * t3523;
    let t24496 = 0.10389515463408878255e3_f64 * t1196 * t24494;
    let t24498 = t1179 * t24407 * t1188;
    let t24500 = 0.5848223622634646207e0_f64 * t1196 * t24498;
    let t24501 = t6752 * t1832;
    let t24509 = t3737 * t1828 * t6744;
    let t24514 = t1774 * t6744;
    (t24496, t24500, t24501, t24509, t24514)
}
