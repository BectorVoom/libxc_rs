//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 901/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk901(t322: f64, t9675: f64, t2941: f64, t833: f64, t1299: f64, t2944: f64, t829: f64, t1013: f64, t2394: f64, t1300: f64, t2397: f64, t327: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t9676 = piecewise3(t324, 0.0_f64, t9675);
    let t9679 = t2941 * t833;
    let t9684 = t2944 * t1299;
    let t9687 = t2944 * t829;
    let t9690 = t1013 * t2394;
    let t9693 = t2941 * t829;
    let t9698 = -0.64e0_f64 * t9676 * t327 - 0.128e1_f64 * t9679 * t829 - 0.256e1_f64 * t2397 * t2394 - 0.384e1_f64 * t9684 * t829 - 0.384e1_f64 * t6693 * t9687 - 0.256e1_f64 * t1300 * t9690 - 0.128e1_f64 * t1300 * t9693 - 0.64e0_f64 * t834 * t9676;
    (t9676, t9687, t9690, t9693, t9698)
}
