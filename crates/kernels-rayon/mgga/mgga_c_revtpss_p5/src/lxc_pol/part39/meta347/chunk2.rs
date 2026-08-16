//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1171/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1171(t3889: f64, t5651: f64, t13716: f64, t1394: f64, t13892: f64, t13902: f64, t13907: f64, t13911: f64, t1392: f64, t1395: f64, t1877: f64, t1879: f64, t4045: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64) -> f64 {
    let t13914 = t5651 * t3889;
    let t13917 = t1394 * t13716;
    let t13920 = -t13892 * t541 - 24.0_f64 * t13902 * t5652 + 60.0_f64 * t13907 * t5650 - 24.0_f64 * t13911 * t5650 - 12.0_f64 * t13914 * t5650 + 3.0_f64 * t13917 * t539 + 6.0_f64 * t1392 * t5655 + 6.0_f64 * t1395 * t5644 - 12.0_f64 * t1877 * t4050 + 3.0_f64 * t1877 * t4053 + 3.0_f64 * t1879 * t4045;
    t13920
}
