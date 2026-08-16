//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1011;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta222(t10638: f64, t231: f64, t2710: f64, t2793: f64, t9285: f64, t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64, t251: f64, t2722: f64, t2723: f64, t4503: f64, t2782: f64, t2760: f64, t822: f64, t2718: f64, t860: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10639, t10645, t10647, t10651, t10652) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1011(t10638, t231, t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t251, t2722);
        let (t10654, t10655, t10657, t10661, t10665) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1012(t10652, t2723, t4503, t2782, t2760, t822, t2718, t860, t2722, t836);
    (t10639, t10645, t10647, t10651, t10652, t10654, t10655, t10657, t10661, t10665)
}
