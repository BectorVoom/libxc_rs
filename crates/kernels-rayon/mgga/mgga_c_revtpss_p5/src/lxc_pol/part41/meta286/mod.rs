//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta286(t820: f64, t823: f64, t844: f64, t2681: f64, t839: f64, t222: f64, t9727: f64, t2737: f64, t9802: f64, t2482: f64, t596: f64, t2487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10811, t10815, t10816, t10824, t10826, t10845, t10846) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1043(t820, t823, t844, t2681, t839, t222, t9727, t2737, t9802, t2482, t596, t2487);
    (t10811, t10815, t10816, t10824, t10826, t10845, t10846)
}
