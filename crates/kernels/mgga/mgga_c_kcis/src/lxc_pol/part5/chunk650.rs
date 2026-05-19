//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 650/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk650<F: Float>(t1103: F, t1104: F, t167: F, t1098: F, t1758: F, t1102: F, t3253: F, t3256: F, t3258: F, t3260: F, t4563: F, t4568: F, t4572: F, t4576: F, t4582: F, t4587: F) -> (F, F, F) {
    let t4589 = t1103 * t1104 * t167;
    let t4592 = t1098 * t1758;
    let t4594 = -t3253 + F::cast_from(0.43802864444444444445e-3_f64) * t3256 + F::new(0.98556445e-3) * t3258 - F::cast_from(0.65704296666666666667e-3_f64) * t3260 + F::cast_from(0.43802864444444444445e-3_f64) * t4563 + F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t4568 + F::new(0.98556445e-3) * t1102 * t4572 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t4576 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t4582 + F::cast_from(0.13140859333333333333e-2_f64) * t4587 * t4589 + F::new(0.98556445e-3) * t4592;
    (t4589, t4592, t4594)
}
