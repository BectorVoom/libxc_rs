//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1346;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta370(t40178: f64, t40067: f64, t40072: f64, t40155: f64, t40157: f64, t40160: f64, t40163: f64, t40167: f64, t40171: f64, t40173: f64, t40175: f64, t39909: f64, t738: f64, t745: f64, t760: f64, t2251: f64, t2609: f64, t2611: f64, t36: f64, t716: f64, t10440: f64, t39875: f64, t9417: f64, t2596: f64, t39871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40179, t40180, t40182) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1346(t40178, t40067, t40072, t40155, t40157, t40160, t40163, t40167, t40171, t40173, t40175, t39909, t738, t745);
        let (t40184, t40187, t40190, t40192, t40194, t40196) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1347(t40182, t760, t2251, t2609, t2611, t36, t716, t10440, t39875, t745, t9417, t2596, t39871);
    (t40179, t40180, t40182, t40184, t40187, t40190, t40192, t40194, t40196)
}
