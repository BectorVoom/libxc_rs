//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 717/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk717(t22645: f64, t22692: f64, t22717: f64, t22725: f64, t2085: f64, t3787: f64, t22923: f64, t22925: f64, t193: f64, t201: f64, t2056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24071 = 0.16449340668482264365e-1_f64 * t22645;
    let t24099 = 0.16449340668482264365e-1_f64 * t22692;
    let t24108 = 0.12793931631041761173e0_f64 * t22717;
    let t24110 = 0.52089578783527170489e-1_f64 * t22725;
    let t24127 = t3787 * t2085;
    let t24156 = 0.12793931631041761173e0_f64 * t22923;
    let t24157 = 0.52089578783527170489e-1_f64 * t22925;
    let t24191 = t193 * t201 * t2056;
    (t24071, t24099, t24108, t24110, t24127, t24156, t24157, t24191)
}
