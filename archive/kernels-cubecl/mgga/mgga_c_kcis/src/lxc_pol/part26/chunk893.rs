//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 893/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk893<F: Float>(t1336: F, t21170: F, t16115: F, t1907: F, t5541: F, t5574: F, t11543: F, t6954: F, t3856: F, t6986: F, t653: F, t6938: F) -> (F, F, F, F, F, F) {
    let t21172 = F::cast_from(1.0_f64) * t21170 * t1336;
    let t21174 = F::cast_from(2.0_f64) * t16115 * t1907;
    let t21176 = F::cast_from(2.0_f64) * t5541 * t5574;
    let t21178 = F::cast_from(2.0_f64) * t11543 * t6954;
    let t21180 = F::cast_from(1.0_f64) * t3856 * t6986;
    let t21186 = t653 * t6938;
    (t21172, t21174, t21176, t21178, t21180, t21186)
}
