//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1299/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1299(t10039: f64, t869: f64, t689: f64, t2777: f64, t4092: f64, t2439: f64, t3923: f64, t555: f64, t4003: f64, t5744: f64, t2782: f64, t4086: f64, t543: f64) -> (f64, f64, f64, f64) {
    let t10040 = t869 * t10039;
    let t10041 = t689 * t10040;
    let t10043 = t2777 * t4092;
    let t10044 = t2439 * t10043;
    let t10059 = t555 * t3923;
    let t10061 = t5744 * t10059 * t4003;
    let t10062 = t2782 * t10061;
    let t10065 = t4086 * t10059 * t543;
    (t10041, t10044, t10062, t10065)
}
