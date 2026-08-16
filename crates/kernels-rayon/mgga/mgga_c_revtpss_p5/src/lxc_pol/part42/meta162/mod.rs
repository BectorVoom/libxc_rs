//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk705;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta162(t225: f64, t4376: f64, t4407: f64, t227: f64, t73: f64, t1544: f64, t853: f64, t775: f64, t4343: f64, t832: f64, t1553: f64, t1555: f64, t229: f64, t830: f64, t833: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4409, t4415, t4416, t4417, t4420, t4423) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk705(t225, t4376, t4407, t227, t73, t1544, t853, t775, t4343, t832, t1553, t1555, t229, t830, t833);
        let t4424 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk706(t231, t4423);
    (t4409, t4415, t4416, t4417, t4420, t4423, t4424)
}
