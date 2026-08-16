//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta424(t18616: f64, t827: f64, t828: f64, t221: f64, t2485: f64, t6017: f64, t2484: f64, t125: f64, t5962: f64, t2747: f64, t837: f64, t2723: f64, t4423: f64, t4364: f64, t4365: f64, t231: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18618, t18622, t18623, t18627, t18629, t18632) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1812(t18616, t827, t828, t221, t2485, t6017, t2484, t125, t5962, t2747, t837, t2723, t4423);
        let (t18634, t18637) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1813(t18632, t4364, t4365, t231, t4343);
    (t18618, t18622, t18623, t18627, t18629, t18632, t18634, t18637)
}
