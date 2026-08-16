//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 864/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk864(t1041: f64, t15731: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t1647: f64, t3140: f64, t3149: f64, t1660: f64, t3201: f64, t1086: f64, t4746: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15732 = t1041 * t15731;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15822 = t1647 * t3140;
    let t15823 = t15822 * t3149;
    let t15862 = t1660 * t3201;
    let t15925 = t4746 * t1086;
    (t15732, t15749, t15750, t15822, t15823, t15862, t15925)
}
