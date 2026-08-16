//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta318(t3712: f64, t372: f64, t3630: f64, t12705: f64, t5341: f64, t3720: f64, t5333: f64, t1263: f64, t675: f64, t1122: f64, t247: f64, t1261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12867, t12868, t12871, t12872, t12875, t12876, t12879, t12881, t12882) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1228(t3712, t372, t3630, t12705, t5341, t3720, t5333, t1263, t675, t1122, t247, t1261);
    (t12867, t12868, t12871, t12872, t12875, t12876, t12879, t12881, t12882)
}
