//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 968/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk968(t11024: f64, t10983: f64, t10988: f64, t10991: f64, t10996: f64, t11001: f64, t11006: f64, t11008: f64, t11014: f64, t11018: f64, t11022: f64, t10614: f64, t10618: f64, t10621: f64, t10625: f64, t10629: f64, t10633: f64, t10637: f64, t10643: f64, t10653: f64, t10657: f64, t10925: f64, t10975: f64) -> (f64, f64) {
    let t11025 = 3.0_f64 / 2.0_f64 * t11024;
    let t11026 = -t10983 - t10988 + t10991 + t10996 - t11001 + t11006 - 0.81300399444200075504e-3_f64 * t11008 + t11014 - t11018 - t11022 - t11025;
    let t11028 = -t10614 + 0.15243824895787514157e-3_f64 * t10643 + t10618 - t10621 + t10625 - t10629 - t10633 + 0.72042316457491791906e-3_f64 * t10653 + t10637 - t10657 + t10925 + t10975 + t11026;
    (t11025, t11028)
}
