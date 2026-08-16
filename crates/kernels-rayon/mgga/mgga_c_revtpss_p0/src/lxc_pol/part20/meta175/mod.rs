//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk914;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta175(t9570: f64, t2626: f64, t676: f64, t3869: f64, t2434: f64, t762: f64, t1331: f64, t3860: f64, t1320: f64, t3855: f64, t186: f64, t685: f64, t793: f64, t1337: f64, t4135: f64, t5541: f64, t7315: f64, t9514: f64, t9517: f64, t9521: f64, t9560: f64, t9562: f64, t9565: f64, t9567: f64, t9569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9571, t9572, t9574, t9575, t9577, t9579, t9581, t9586) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk914(t9570, t2626, t676, t3869, t2434, t762, t1331, t3860, t1320, t3855, t186, t685, t793);
        let (t9588, t9589) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk915(t1337, t9586, t4135, t5541, t7315, t9514, t9517, t9521, t9560, t9562, t9565, t9567, t9569, t9571, t9574, t9577, t9579, t9581);
    (t9571, t9572, t9574, t9575, t9577, t9579, t9581, t9586, t9588, t9589)
}
