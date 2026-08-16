//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta853 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2739;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2740;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta853(t17609: f64, t5265: f64, t17544: f64, t5274: f64, t1222: f64, t17471: f64, t20298: f64, t20302: f64, t1260: f64, t57465: f64, t21334: f64, t17763: f64, t5378: f64, t12855: f64, t12916: f64, t20977: f64, t20913: f64, t3172: f64, t3711: f64, t21107: f64, t3704: f64, t17628: f64, t5373: f64, t20851: f64, t3678: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71550, t71552, t71571, t71582, t71585, t71590, t71598) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2739(t17609, t5265, t17544, t5274, t1222, t17471, t20298, t20302, t1260, t57465, t21334, t17763, t5378);
        let (t71630, t71687, t71710, t71718, t71738) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2740(t12855, t12916, t20977, t20913, t3172, t3711, t21107, t3704, t17628, t5373, t20851, t3678);
    (t71550, t71552, t71571, t71582, t71585, t71590, t71598, t71630, t71687, t71710, t71718, t71738)
}
