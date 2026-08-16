//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1718;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta464(t212: f64, t7506: f64, t1358: f64, t689: f64, t2097: f64, t785: f64, t2439: f64, t2435: f64, t7493: f64, t26069: f64, t26277: f64, t26072: f64, t7515: f64, t25924: f64, t4077: f64, t2027: f64, t213: f64, t25921: f64, t25930: f64, t26294: f64, t26295: f64, t26302: f64, t26305: f64, t26309: f64, t26335: f64, t26338: f64, t26343: f64, t26347: f64, t26351: f64, t4078: f64, t561: f64, t7295: f64, t7511: f64, t7523: f64, t7528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26354, t26355, t26356, t26358, t26359, t26361, t26363, t26365, t26366) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1718(t212, t7506, t1358, t689, t2097, t785, t2439, t2435, t7493, t26069, t26277, t26072, t7515);
        let (t26371, t26374) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1719(t2097, t25924, t4077, t2027, t213, t25921, t25930, t26294, t26295, t26302, t26305, t26309, t26335, t26338, t26343, t26347, t26351, t26356, t26361, t26363, t26365, t26366, t4078, t561, t7295, t7511, t7523, t7528);
    (t26354, t26355, t26356, t26358, t26359, t26361, t26363, t26365, t26366, t26371, t26374)
}
