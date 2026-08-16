//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1796;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1797;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta494(t1416: f64, t25978: f64, t3999: f64, t64: f64, t239: f64, t820: f64, t240: f64, t7262: f64, t3994: f64, t2661: f64, t2482: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25979, t25981) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1796(t1416, t25978, t3999, t64);
        let (t25983, t25986) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1797(t239, t25981, t820, t240, t7262);
        let (t25987, t25988, t25997) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1798(t25986, t3994, t2661, t2482, t27, t7262);
    (t25979, t25981, t25983, t25986, t25987, t25988, t25997)
}
