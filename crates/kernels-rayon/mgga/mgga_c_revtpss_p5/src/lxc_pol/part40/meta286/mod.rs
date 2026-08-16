//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta286(t10240: f64, t10: f64, t580: f64, t22: f64, t576: f64, t15: f64, t588: f64, t11: f64, t2: f64, t2224: f64, t27: f64, t584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10241, t10270, t10272, t10275, t10278, t10279, t10281) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1035(t10240, t10, t580, t22, t576, t15, t588, t11, t2, t2224, t27, t584);
    (t10241, t10270, t10272, t10275, t10278, t10279, t10281)
}
