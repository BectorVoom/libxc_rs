//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk755;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk756;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk757;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta135(t3545: f64, t456: f64, t1197: f64, t135: f64, t1174: f64, t1176: f64, t3247: f64, t3242: f64, t3439: f64, t121: f64, t486: f64, t1216: f64, t248: f64, t1213: f64, t478: f64, t483: f64, t3068: f64, t1244: f64, t1230: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3547, t3548, t3549, t3555, t3560, t3570) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk755(t3545, t456, t1197, t135, t1174, t1176, t3247, t3242, t3439, t121, t486);
        let t3572 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk756(t1216, t248, t3570);
        let (t3573, t3575, t3576, t3577) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk757(t1213, t3572, t478, t483, t3068, t1244);
        let t3578 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk758(t1230, t820);
    (t3547, t3548, t3549, t3555, t3560, t3570, t3572, t3573, t3575, t3576, t3577, t3578)
}
