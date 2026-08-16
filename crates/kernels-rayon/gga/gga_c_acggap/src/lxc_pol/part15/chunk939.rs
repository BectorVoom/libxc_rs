//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 939/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk939(t31229: f64, t31283: f64, t31290: f64, t31295: f64, t31340: f64, t31376: f64, t31380: f64, t31389: f64, t31391: f64, t31406: f64, t31470: f64, t31472: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32740 = 0.12862205435420921092e-2_f64 * t31229;
    let t32760 = 0.64311027177104605458e-3_f64 * t31283;
    let t32763 = 0.39624596284901231606e-1_f64 * t31290;
    let t32765 = 0.18868855373762491242e-2_f64 * t31295;
    let t32782 = 0.114609375e0_f64 * t31340;
    let t32795 = 1309.0_f64 / 2592.0_f64 * t31376;
    let t32796 = 455.0_f64 / 648.0_f64 * t31380;
    let t32799 = 0.25724410870841842183e-2_f64 * t31389;
    let t32800 = 0.37737710747524982482e-2_f64 * t31391;
    let t32803 = 0.18864567971950684268e-1_f64 * t31406;
    let t32823 = 0.77173232612525526551e-2_f64 * t31470;
    let t32824 = 0.38586616306262763276e-2_f64 * t31472;
    (t32740, t32760, t32763, t32765, t32782, t32795, t32796, t32799, t32800, t32803, t32823, t32824)
}
