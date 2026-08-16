//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1436;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta251(t3869: f64, t9575: f64, t1331: f64, t3860: f64, t186: f64, t685: f64, t793: f64, t1337: f64, t4146: f64, t565: f64, t1333: f64, t30: f64, t513: f64, t33: f64, t516: f64, t2435: f64, t3900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9577, t9578, t9586) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1436(t3869, t9575, t1331, t3860, t186, t685, t793);
        let (t9588, t9593, t9597, t9598, t9603, t9605, t9615, t9617, t9632) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1437(t1337, t9586, t4146, t565, t1333, t3860, t30, t513, t33, t516, t2435, t3900);
    (t9577, t9578, t9586, t9588, t9593, t9597, t9598, t9603, t9605, t9615, t9617, t9632)
}
