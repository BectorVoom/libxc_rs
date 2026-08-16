//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta998 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3389;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta998(t11294: f64, t19331: f64, t19127: f64, t2926: f64, t2924: f64, t934: f64, t3007: f64, t6226: f64, t981: f64, t4631: f64, t15543: f64, t4719: f64, t1634: f64, t52877: f64, t63597: f64, t11299: f64, t2875: f64, t6110: f64, t15101: f64, t15383: f64, t63633: f64, t63636: f64, t63638: f64, t63641: f64, t63644: f64, t63647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63649, t63653, t63656, t63657, t63660, t63662) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3389(t11294, t19331, t19127, t2926, t2924, t934, t3007, t6226, t981, t4631, t15543, t4719);
        let (t63665, t63668, t63670, t63671) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3390(t1634, t52877, t63597, t11299, t2875, t6110, t15101, t15383, t63633, t63636, t63638, t63641, t63644, t63647, t63649, t63653, t63656, t63660, t63662);
    (t63649, t63653, t63656, t63657, t63660, t63662, t63665, t63668, t63670, t63671)
}
