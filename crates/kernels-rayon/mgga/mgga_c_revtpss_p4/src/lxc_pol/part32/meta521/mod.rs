//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta521(t221: f64, t22274: f64, t22279: f64, t22287: f64, t1398: f64, t6843: f64, t1883: f64, t5658: f64, t543: f64, t73842: f64, t73820: f64, t6862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74419, t74423, t74477, t74700, t75012, t75016, t75047, t75051) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1824(t221, t22274, t22279, t22287, t1398, t6843, t1883, t5658, t543, t73842, t73820, t6862);
    (t74419, t74423, t74477, t74700, t75012, t75016, t75047, t75051)
}
