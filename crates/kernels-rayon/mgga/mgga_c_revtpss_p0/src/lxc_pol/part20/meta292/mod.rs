//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta292(t3515: f64, t3520: f64, t5206: f64, t1196: f64, t1129: f64, t3431: f64, t408: f64, t1149: f64, t3385: f64, t3434: f64, t421: f64, t1187: f64, t3495: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12222, t12224, t12226, t12227, t12228, t12230, t12231, t12233, t12234) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1163(t3515, t3520, t5206, t1196, t1129, t3431, t408, t1149, t3385, t3434, t421, t1187, t3495);
    (t12222, t12224, t12226, t12227, t12228, t12230, t12231, t12233, t12234)
}
