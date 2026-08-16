//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1958/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1958(t1358: f64, t212: f64, t28888: f64, t689: f64, t25898: f64, t8099: f64, t94849: f64, t26277: f64, t97916: f64, t97799: f64, t2439: f64, t785: f64, t8085: f64) -> (f64, f64, f64, f64, f64) {
    let t102129 = 0.10975748638225852664e-1_f64 * t689 * t212 * t28888 * t1358;
    let t102131 = t94849 * t25898 * t8099;
    let t102133 = t97916 * t26277;
    let t102135 = t97799 * t26277;
    let t102139 = t2439 * t785 * t8085 * t1358;
    (t102129, t102131, t102133, t102135, t102139)
}
