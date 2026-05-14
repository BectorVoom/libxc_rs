//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 844/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk844<F: Float>(t13973: F, t274: F, t3589: F, t4740: F, t13908: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t9700: F, t9702: F, t9708: F, t9710: F, t9712: F, t13712: F) -> (F, F, F, F) {
    let t13974 = t13973 * t274;
    let t13977 = t4740 * t3589;
    let t14001 = 0.22076e0 * t13908;
    let t14002 = -0.20128333333333333334e0 * t9700 - 0.11038e0 * t9702 - 0.18396666666666666667e0 * t9708 + 0.5519e-1 * t9710 + 0.18396666666666666667e-1 * t9712 - 0.20128333333333333333e0 * t13729 - 0.33547222222222222222e0 * t13720 - 0.80513333333333333333e0 * t13726 + 0.60385e0 * t13738 + 0.24154e1 * t13735 - t14001;
    let t14015 = 0.13418888888888888889e0 * t13712;
    (t13974, t13977, t14002, t14015)
}
