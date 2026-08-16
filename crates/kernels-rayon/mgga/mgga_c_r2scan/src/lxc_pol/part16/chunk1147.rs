//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1147/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1147(t1013: f64, t1074: f64, t11066: f64, t11894: f64, t12602: f64, t12611: f64, t12614: f64, t12617: f64, t1300: f64, t19203: f64, t2394: f64, t2941: f64, t2944: f64, t3370: f64, t3633: f64, t3638: f64, t6693: f64, t829: f64, t9676: f64, t9693: f64) -> f64 {
    let t42592 = -0.384e1_f64 * t11066 * t9693 - 0.384e1_f64 * t6693 * t3370 * t2944 - 0.256e1_f64 * t1300 * t11894 * t1013 - 0.256e1_f64 * t1300 * t3633 * t2394 - 0.128e1_f64 * t1300 * t3370 * t2941 - 0.128e1_f64 * t1300 * t1074 * t9676 - 0.128e1_f64 * t1300 * t12602 * t829 - 0.768e1_f64 * t6693 * t3638 * t2394 - 0.1536e2_f64 * t19203 * t12611 * t829 - 0.768e1_f64 * t6693 * t12614 * t829 - 0.384e1_f64 * t6693 * t12617 * t829;
    t42592
}
