//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 936/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk936(t10760: f64, t6087: f64, t6085: f64, t6064: f64, t6093: f64, t2096: f64, t571: f64, t572: f64) -> (f64, f64, f64, f64, f64) {
    let t10761 = t10760 * t6087;
    let t10762 = t6085 * t10761;
    let t10764 = t10760 * t6064;
    let t10765 = t6093 * t10764;
    let t10768 = t571 * t572 * t2096;
    (t10761, t10762, t10764, t10765, t10768)
}
