//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2712;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta764(t4186: f64, t4401: f64, t606: f64, t749: f64, t39737: f64, t39766: f64, t10433: f64, t4311: f64, t10489: f64, t2403: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39764: f64, t39770: f64, t4546: f64, t1522: f64, t40158: f64, t14362: f64, t9575: f64, t123: f64, t2630: f64, t4392: f64, t4398: f64, t9318: f64, t11231: f64, t14330: f64, t4402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49912, t49913, t49918, t49920, t49921) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2712(t4186, t4401, t606, t749, t39737, t39766, t10433, t4311, t10489, t2403, t39741, t39744, t39747, t39750, t39756, t39760, t39764, t39770, t4546);
        let (t49925, t49927, t49930, t49941, t49944) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2713(t1522, t40158, t14362, t9575, t123, t2630, t4392, t4398, t9318, t11231, t14330, t4402);
    (t49912, t49913, t49918, t49920, t49921, t49925, t49927, t49930, t49941, t49944)
}
