//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1015/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1015(t1864: f64, t23992: f64, t1860: f64, t22819: f64, t22825: f64, t22858: f64, t22863: f64, t22867: f64, t22645: f64, t22692: f64, t22717: f64, t22725: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23993 = t23992 * t1864;
    let t23995 = 88.0_f64 / 27.0_f64 * t1860 * t23993;
    let t24049 = 0.33643963411783659044e-4_f64 * t22819;
    let t24050 = 0.10541775202358879834e-2_f64 * t22825;
    let t24058 = 119.0_f64 / 3456.0_f64 * t22858;
    let t24060 = 35.0_f64 / 216.0_f64 * t22863;
    let t24061 = 0.22608743412718618878e-1_f64 * t22867;
    let t24071 = 0.16449340668482264365e-1_f64 * t22645;
    let t24099 = 0.16449340668482264365e-1_f64 * t22692;
    let t24108 = 0.12793931631041761173e0_f64 * t22717;
    let t24110 = 0.52089578783527170489e-1_f64 * t22725;
    (t23993, t23995, t24049, t24050, t24058, t24060, t24061, t24071, t24099, t24108, t24110)
}
