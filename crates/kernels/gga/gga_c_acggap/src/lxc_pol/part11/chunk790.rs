//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 790/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk790<F: Float>(t30183: F, t1967: F, t7784: F, t1200: F, t7614: F, t30169: F, t601: F, t3646: F, t597: F, t606: F, t1979: F, t980: F, t1994: F, t7736: F, t993: F, t7472: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30184 = 0.25724410870841842183e-2 * t30183;
    let t30185 = t1967 * t7784;
    let t30187 = t7614 * t1200;
    let t30191 = t30169 * t601;
    let t30192 = 0.13505315707191967146e-1 * t30191;
    let t30193 = t3646 * t597;
    let t30194 = t30193 * t606;
    let t30195 = 0.15724046144802076034e-3 * t30194;
    let t30196 = t980 * t1979;
    let t30197 = t30196 * t1994;
    let t30198 = 0.31448092289604152067e-3 * t30197;
    let t30199 = t7736 * t993;
    let t30200 = 0.12862205435420921092e-2 * t30199;
    let t30201 = t1967 * t7472;
    (t30184, t30185, t30187, t30192, t30193, t30195, t30196, t30198, t30200, t30201)
}
