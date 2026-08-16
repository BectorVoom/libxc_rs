//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1473;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta419(t31303: f64, t31326: f64, t3: f64, t2178: f64, t670: f64, t1518: f64, t31117: f64, t4292: f64, t8295: f64, t116: f64, t8362: f64, t117: f64, t31292: f64, param_d: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t8289: f64, t8296: f64, t8299: f64, t8377: f64, t8383: f64, t8386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31328, t31329, t31340, t31358, t31359, t31362, t31365, t31370, t31371, t31374) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1473(t31303, t31326, t3, t2178, t670, t1518, t31117, t4292, t8295, t116, t8362, t117, t31292, param_d);
        let t31377 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1474(t1459, t1461, t1916, t1918, t2187, t2189, t31340, t31359, t31362, t31365, t31371, t31374, t572, t573, t5795, t5802, t5805, t8289, t8296, t8299, t8377, t8383, t8386);
    (t31328, t31329, t31340, t31358, t31359, t31362, t31365, t31370, t31371, t31374, t31377)
}
