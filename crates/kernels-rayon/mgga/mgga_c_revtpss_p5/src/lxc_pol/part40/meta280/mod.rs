//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta280(t10019: f64, t4101: f64, t555: f64, t5744: f64, t786: f64, t3923: f64, t675: f64, t268: f64, t4003: f64, t2435: f64, t4093: f64, t4083: f64, t9303: f64, t4066: f64, t545: f64, t869: f64, t689: f64, t2777: f64, t4092: f64, t2439: f64, t2782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10020, t10022, t10024, t10027, t10032, t10035) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1025(t10019, t4101, t555, t5744, t786, t3923, t675, t268, t4003, t2435, t4093, t4083, t9303);
        let (t10041, t10044, t10059, t10062) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1026(t4066, t545, t869, t689, t2777, t4092, t2439, t3923, t555, t4003, t5744, t2782);
    (t10020, t10022, t10024, t10027, t10032, t10035, t10041, t10044, t10059, t10062)
}
