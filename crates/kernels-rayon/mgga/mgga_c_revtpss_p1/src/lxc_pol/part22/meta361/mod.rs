//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1882;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1883;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta361(t1175: f64, t3495: f64, t3515: f64, t3523: f64, t1156: f64, t3451: f64, t12295: f64, t12351: f64, t1178: f64, t3519: f64, t439: f64, t3522: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12491, t12500, t12511, t12542, t12543, t12552) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1882(t1175, t3495, t3515, t3523, t1156, t3451, t12295, t12351, t1178, t3519);
        let t12553 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1883(t12552, t439);
        let t12555 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1884(t3522, t447);
    (t12491, t12500, t12511, t12542, t12543, t12552, t12553, t12555)
}
