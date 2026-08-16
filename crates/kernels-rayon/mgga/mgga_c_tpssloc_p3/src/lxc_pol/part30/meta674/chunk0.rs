//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2103/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2103(t90980: f64, t90993: f64, t91000: f64, t91149: f64, t91167: f64, t91305: f64, t91312: f64, t91394: f64, t91398: f64, t91078: f64, t91081: f64, t91531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93595 = 0.16449340668482264365e-1_f64 * t90980;
    let t93605 = 0.16449340668482264365e-1_f64 * t90993;
    let t93615 = 0.12793931631041761173e0_f64 * t91000;
    let t93650 = 119.0_f64 / 864.0_f64 * t91149;
    let t93656 = 0.22608743412718618878e-1_f64 * t91167;
    let t93721 = 119.0_f64 / 3456.0_f64 * t91305;
    let t93723 = 0.10541775202358879834e-2_f64 * t91312;
    let t93757 = 119.0_f64 / 3456.0_f64 * t91394;
    let t93760 = 35.0_f64 / 108.0_f64 * t91398;
    let t93795 = 0.52089578783527170489e-1_f64 * t91078;
    let t93796 = 0.3289868133696452873e-1_f64 * t91081;
    let t93899 = 0.52089578783527170489e-1_f64 * t91531;
    (t93595, t93605, t93615, t93650, t93656, t93721, t93723, t93757, t93760, t93795, t93796, t93899)
}
