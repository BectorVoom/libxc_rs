//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1593/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1593(t23095: f64, t23105: f64, t23107: f64, t23140: f64, t23143: f64, t23013: f64, t23031: f64, t2047: f64, t2627: f64, t23173: f64, t7084: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24218 = 0.10541775202358879834e-2_f64 * t23095;
    let t24220 = 0.33643963411783659044e-4_f64 * t23105;
    let t24221 = 119.0_f64 / 3456.0_f64 * t23107;
    let t24230 = 0.22608743412718618878e-1_f64 * t23140;
    let t24231 = 35.0_f64 / 216.0_f64 * t23143;
    let t24246 = 0.12793931631041761173e0_f64 * t23013;
    let t24250 = 0.52089578783527170489e-1_f64 * t23031;
    let t24255 = t2627 * t2047;
    let t24265 = 0.16449340668482264365e-1_f64 * t23173;
    let t24269 = t814 * t7084;
    (t24218, t24220, t24221, t24230, t24231, t24246, t24250, t24255, t24265, t24269)
}
