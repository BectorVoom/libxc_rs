//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta401(t40135: f64, t760: f64, t39875: f64, t39894: f64, t9371: f64, t39960: f64, t39963: f64, t39909: f64, t738: f64, t745: f64, t9417: f64, t2596: f64, t39871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40137, t40165, t40167, t40169, t40171, t40182, t40184, t40192, t40194, t40196) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1336(t40135, t760, t39875, t39894, t9371, t39960, t39963, t39909, t738, t745, t9417, t2596, t39871);
    (t40137, t40165, t40167, t40169, t40171, t40182, t40184, t40192, t40194, t40196)
}
