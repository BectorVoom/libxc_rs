//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta522(t13790: f64, t5658: f64, t1398: f64, t23037: f64, t543: f64, t74700: f64, t116: f64, t21813: f64, t5966: f64, t890: f64, t5962: f64, t1544: f64, t4537: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t75188, t75267, t75305, t75439, t77408, t77425, t77441) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1825(t13790, t5658, t1398, t23037, t543, t74700, t116, t21813, t5966, t890, t5962, t1544, t4537);
    (t75188, t75267, t75305, t75439, t77408, t77425, t77441)
}
