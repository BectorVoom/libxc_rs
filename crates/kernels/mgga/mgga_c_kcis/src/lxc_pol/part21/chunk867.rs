//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 867/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk867<F: Float>(t1021: F, t13346: F, t1020: F, t1121: F, t167: F, t3203: F, t3202: F, t13172: F, t13145: F, t13332: F, t13337: F, t13340: F, t13344: F, t2836: F, t9557: F, t9559: F, t9563: F, t9572: F) -> (F, F, F, F) {
    let t13347 = t1021 * t13346;
    let t13348 = t1020 * t13347;
    let t13353 = t167 * t1121;
    let t13354 = t3203 * t13353;
    let t13355 = t3202 * t13354;
    let t13356 = t13172 * t13355;
    let t13359 = -F::new(0.22109259259259259258e-2) * t13332 + F::new(0.890445125e-2) * t2836 * t13145 - F::new(0.33163888888888888888e-2) * t13337 - F::new(0.11054629629629629629e-2) * t13340 + F::new(0.88437037037037037034e-2) * t13344 + F::new(0.1621345679012345679e-1) * t13348 - F::new(0.22109259259259259258e-2) * t9557 - F::new(0.58958024691358024689e-2) * t9559 - F::new(0.73697530864197530861e-3) * t9563 - F::new(0.66327777777777777776e-2) * t13356 + F::new(0.22109259259259259258e-2) * t9572;
    (t13348, t13354, t13356, t13359)
}
