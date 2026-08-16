//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1020/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1020(t322: f64, t12601: f64, t1074: f64, t2944: f64, t1013: f64, t3633: f64, t2941: f64, t11066: f64, t11897: f64, t1300: f64, t327: f64, t3373: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t12602 = piecewise3(t324, 0.0_f64, t12601);
    let t12611 = t1074 * t2944;
    let t12614 = t3633 * t1013;
    let t12617 = t1074 * t2941;
    let t12622 = -0.64e0_f64 * t12602 * t327 - 0.256e1_f64 * t11897 * t1013 - 0.384e1_f64 * t11066 * t2944 - 0.128e1_f64 * t3373 * t2941 - 0.384e1_f64 * t6693 * t12611 - 0.256e1_f64 * t1300 * t12614 - 0.128e1_f64 * t1300 * t12617 - 0.64e0_f64 * t834 * t12602;
    (t12602, t12611, t12614, t12617, t12622)
}
