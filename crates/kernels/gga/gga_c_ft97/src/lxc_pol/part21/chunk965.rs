//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 965/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk965<F: Float>(t1969: F, t4458: F, t5773: F, t12664: F, t6708: F, t27165: F, t925: F, t5899: F, t4462: F, t5900: F, t4454: F, t9049: F, t1359: F, t4753: F, t586: F, t23609: F, t28: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30169 = t1969 * t5773 * t4458;
    let t30172 = t12664 * t6708;
    let t30175 = t1969 * t27165 * t925;
    let t30176 = t5899 * t30175;
    let t30179 = t1969 * t5900 * t4462;
    let t30180 = t5899 * t30179;
    let t30183 = t9049 * t5900 * t4454;
    let t30184 = t5899 * t30183;
    let t30186 = t1359 * t4753;
    let t30187 = t586 * t30186;
    let t30189 = t23609 * t28 * t30187;
    (t30169, t30172, t30175, t30176, t30179, t30180, t30183, t30184, t30186, t30187, t30189)
}
