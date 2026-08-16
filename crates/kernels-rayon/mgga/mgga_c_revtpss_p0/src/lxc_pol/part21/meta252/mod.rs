//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1439;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1440;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta252(t3869: f64, t9572: f64, t2434: f64, t762: f64, t1331: f64, t3860: f64, t1320: f64, t3855: f64, t186: f64, t685: f64, t793: f64, t1337: f64, t4135: f64, t5541: f64, t7315: f64, t9514: f64, t9517: f64, t9521: f64, t9560: f64, t9562: f64, t9565: f64, t9567: f64, t9569: f64, t9571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9574, t9575) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1439(t3869, t9572, t2434, t762);
        let (t9577, t9578, t9579, t9580, t9581, t9586) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1440(t3869, t9575, t1331, t3860, t1320, t3855, t186, t685, t793);
        let (t9588, t9589) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1441(t1337, t9586, t4135, t5541, t7315, t9514, t9517, t9521, t9560, t9562, t9565, t9567, t9569, t9571, t9574, t9577, t9579, t9581);
    (t9574, t9575, t9577, t9578, t9579, t9580, t9581, t9586, t9588, t9589)
}
