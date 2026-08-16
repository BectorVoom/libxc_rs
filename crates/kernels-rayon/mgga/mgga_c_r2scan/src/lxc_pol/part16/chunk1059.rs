//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1059/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1059(t37560: f64, t10949: f64, t2312: f64, t3446: f64, t3447: f64, t3438: f64, t6868: f64, t10966: f64, t1103: f64, t269: f64, t607: f64, t10707: f64, t2195: f64) -> (f64, f64, f64, f64, f64) {
    let t37561 = 0.12195059916630011326e-2_f64 * t37560;
    let t37564 = t3446 * t3447 * t10949 * t2312;
    let t37568 = t3446 * t3447 * t3438 * t6868;
    let t37569 = 0.15243824895787514157e-3_f64 * t37568;
    let t37580 = t10966 * t1103 * t607 * t269;
    let t37582 = t2195 * t10707;
    (t37561, t37564, t37569, t37580, t37582)
}
