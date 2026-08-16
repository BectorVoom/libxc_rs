//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta560(t1843: f64, t5920: f64, t1513: f64, t5891: f64, t10208: f64, t4263: f64, t5915: f64, t1504: f64, t5895: f64, t10227: f64, t4269: f64, t5823: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22578, t22589, t22590, t22593, t22596, t22597, t22600) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2125(t1843, t5920, t1513, t5891, t10208, t4263, t5915, t1504, t5895, t10227, t4269, t5823);
    (t22578, t22589, t22590, t22593, t22596, t22597, t22600)
}
