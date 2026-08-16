//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1108/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1108<F: Float>(t13973: F, t274: F, t3589: F, t4740: F, t13908: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t9700: F, t9702: F, t9708: F, t9710: F, t9712: F) -> (F, F, F) {
    let t13974 = t13973 * t274;
    let t13977 = t4740 * t3589;
    let t14001 = F::cast_from(0.22076e0_f64) * t13908;
    let t14002 = -F::cast_from(0.20128333333333333334e0_f64) * t9700 - F::cast_from(0.11038e0_f64) * t9702 - F::cast_from(0.18396666666666666667e0_f64) * t9708 + F::cast_from(0.5519e-1_f64) * t9710 + F::cast_from(0.18396666666666666667e-1_f64) * t9712 - F::cast_from(0.20128333333333333333e0_f64) * t13729 - F::cast_from(0.33547222222222222222e0_f64) * t13720 - F::cast_from(0.80513333333333333333e0_f64) * t13726 + F::cast_from(0.60385e0_f64) * t13738 + F::cast_from(0.24154e1_f64) * t13735 - t14001;
    (t13974, t13977, t14002)
}
