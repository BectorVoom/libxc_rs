//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1253/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1253(t37524: f64, t37528: f64, t43735: f64, t43739: f64, t43742: f64, t43747: f64, t43750: f64, t43752: f64, t43754: f64, t43756: f64, t43892: f64, t43895: f64, t43898: f64, t43902: f64, t43907: f64) -> f64 {
    let t43909 = -0.36021158228745895953e-3_f64 * t43892 - 0.72042316457491791906e-3_f64 * t43895 - 0.72042316457491791906e-3_f64 * t43898 + t43735 + 0.72042316457491791906e-3_f64 * t43902 + 0.36021158228745895953e-3_f64 * t43907 + t43739 + t43742 + t43747 + t43750 + t43752 - t43754 - t43756 + t37524 - t37528;
    t43909
}
