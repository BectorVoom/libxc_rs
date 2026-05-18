//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 939/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk939<F: Float>(t31229: F, t31283: F, t31290: F, t31295: F, t31340: F, t31376: F, t31380: F, t31389: F, t31391: F, t31406: F, t31470: F, t31472: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32740 = F::new(0.12862205435420921092e-2) * t31229;
    let t32760 = F::new(0.64311027177104605458e-3) * t31283;
    let t32763 = F::new(0.39624596284901231606e-1) * t31290;
    let t32765 = F::new(0.18868855373762491242e-2) * t31295;
    let t32782 = F::new(0.114609375e0) * t31340;
    let t32795 = F::new(1309.0) / F::new(2592.0) * t31376;
    let t32796 = F::new(455.0) / F::new(648.0) * t31380;
    let t32799 = F::new(0.25724410870841842183e-2) * t31389;
    let t32800 = F::new(0.37737710747524982482e-2) * t31391;
    let t32803 = F::new(0.18864567971950684268e-1) * t31406;
    let t32823 = F::new(0.77173232612525526551e-2) * t31470;
    let t32824 = F::new(0.38586616306262763276e-2) * t31472;
    (t32740, t32760, t32763, t32765, t32782, t32795, t32796, t32799, t32800, t32803, t32823, t32824)
}
