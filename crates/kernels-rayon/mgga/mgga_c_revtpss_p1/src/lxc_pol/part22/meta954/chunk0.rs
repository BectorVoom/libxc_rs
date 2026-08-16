//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3197/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3197(t1269: f64, t13126: f64, t460: f64, t13147: f64, t1770: f64, t1204: f64, t17852: f64, t1209: f64, t1284: f64, t5412: f64, t17845: f64, t17306: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59945 = t460 * t13126 * t1269;
    let t59948 = t1770 * t13147;
    let t59987 = t1204 * t17852;
    let t60008 = t1209 * t1284 * t5412;
    let t60013 = t1204 * t17845;
    let t60019 = t17306 * t3754;
    (t59945, t59948, t59987, t60008, t60013, t60019)
}
