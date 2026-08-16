//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta464(t1045: f64, t19497: f64, t3117: f64, t1043: f64, t11631: f64, t19450: f64, t4894: f64, t19501: f64, t4910: f64, t11274: f64, t11277: f64, t11789: f64, t11875: f64, t15684: f64, t15906: f64, t16081: f64, t19731: f64, t19738: f64, t19741: f64, t3091: f64, t3115: f64, t4896: f64, t4902: f64, t6308: f64, t6312: f64, t6339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19744, t19745, t19748, t19749, t19750, t19753, t19754, t19757, t19758, t19763) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1905(t1045, t19497, t3117, t1043, t11631, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
    (t19744, t19745, t19748, t19749, t19750, t19753, t19754, t19757, t19758, t19763)
}
