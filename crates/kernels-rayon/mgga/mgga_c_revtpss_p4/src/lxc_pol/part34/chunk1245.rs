//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1245/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1245(t18352: f64, t1945: f64, t807: f64, t29654: f64, t686: f64, t72: f64, t25387: f64, t25375: f64, t29610: f64, t29668: f64, t689: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106102 = t807 * t1945 * t18352;
    let t106120 = t29654 * t72 * t686;
    let t106121 = t25387 * t106120;
    let t106123 = t25375 * t106120;
    let t106128 = t29610 * t72 * t686;
    let t106129 = t25387 * t106128;
    let t106150 = t29668 * t689;
    let t106151 = t25431 * t106150;
    (t106102, t106121, t106123, t106128, t106129, t106150, t106151)
}
