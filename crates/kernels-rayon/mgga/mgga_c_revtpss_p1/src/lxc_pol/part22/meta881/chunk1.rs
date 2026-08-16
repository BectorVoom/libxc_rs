//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3054/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3054(t10073: f64, t14537: f64, t10532: f64, t14598: f64, t231: f64, t50511: f64, t2782: f64, t2797: f64, t10069: f64, t1568: f64, t2645: f64, t2783: f64) -> (f64, f64, f64, f64, f64) {
    let t51688 = t10073 * t14537;
    let t51696 = t14598 * t10532;
    let t51698 = t50511 * t231;
    let t51700 = t2782 * t2797 * t51698;
    let t51703 = t10069 * t14537;
    let t51708 = t2782 * t2783 * t1568 * t2645 * t231;
    (t51688, t51696, t51700, t51703, t51708)
}
