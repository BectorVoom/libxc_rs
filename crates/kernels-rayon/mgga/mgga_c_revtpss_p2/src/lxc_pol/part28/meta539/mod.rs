//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta539(t1936: f64, t670: f64, t1518: f64, t572: f64, t26123: f64, t4292: f64, t7330: f64, t1459: f64, t7953: f64, t116: f64, t7741: f64, t117: f64, t28042: f64, t1461: f64, t1918: f64, t2040: f64, t28246: f64, t28257: f64, t28259: f64, t28261: f64, t28263: f64, t573: f64, t5802: f64, t5805: f64, t7324: f64, t7944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28264, t28265, t28267, t28268, t28270, t28271, t28273, t28275, t28276, t28277, t28279, t28280) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1987(t1936, t670, t1518, t572, t26123, t4292, t7330, t1459, t7953, t116, t7741, t117, t28042);
        let t28283 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1988(t28280, t572, t1461, t1918, t2040, t28246, t28257, t28259, t28261, t28263, t28267, t28270, t28273, t28275, t28279, t573, t5802, t5805, t7324, t7944);
    (t28264, t28265, t28268, t28271, t28276, t28277, t28280, t28283)
}
