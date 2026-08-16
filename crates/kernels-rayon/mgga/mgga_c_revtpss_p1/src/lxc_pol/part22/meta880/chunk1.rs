//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3051/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3051(t10073: f64, t14496: f64, t231: f64, t2782: f64, t2783: f64, t51625: f64, t14946: f64, t2710: f64, t9285: f64, t4469: f64, t836: f64, t14598: f64, t14600: f64, t2434: f64) -> (f64, f64, f64, f64, f64) {
    let t51637 = t10073 * t14496;
    let t51642 = t2782 * t2783 * t51625 * t231;
    let t51646 = t2710 * t14946 * t9285;
    let t51653 = t2782 * t2783 * t4469 * t836 * t231;
    let t51657 = t14598 * t14600 * t2434 * t836;
    (t51637, t51642, t51646, t51653, t51657)
}
