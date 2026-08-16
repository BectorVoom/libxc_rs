//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2087/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2087(t90659: f64, t90663: f64, t90837: f64, t90868: f64, t90900: f64, t90980: f64, t90993: f64, t91000: f64, t91149: f64, t91167: f64, t91305: f64, t91312: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93445 = 0.12793931631041761173e0_f64 * t90659;
    let t93446 = 0.16449340668482264365e-1_f64 * t90663;
    let t93517 = 0.10417915756705434098e0_f64 * t90837;
    let t93538 = 0.12793931631041761173e0_f64 * t90868;
    let t93563 = 0.52089578783527170489e-1_f64 * t90900;
    let t93595 = 0.16449340668482264365e-1_f64 * t90980;
    let t93605 = 0.16449340668482264365e-1_f64 * t90993;
    let t93615 = 0.12793931631041761173e0_f64 * t91000;
    let t93650 = 119.0_f64 / 864.0_f64 * t91149;
    let t93656 = 0.22608743412718618878e-1_f64 * t91167;
    let t93721 = 119.0_f64 / 3456.0_f64 * t91305;
    let t93723 = 0.10541775202358879834e-2_f64 * t91312;
    (t93445, t93446, t93517, t93538, t93563, t93595, t93605, t93615, t93650, t93656, t93721, t93723)
}
