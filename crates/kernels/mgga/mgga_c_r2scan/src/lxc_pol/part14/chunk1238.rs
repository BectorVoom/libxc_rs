//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1238/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1238<F: Float>(t11506: F, t39015: F, t2867: F, t3275: F, t38739: F, t11002: F, t1115: F, t2847: F, t3269: F, t39197: F, t39198: F, t3262: F, t3472: F, t40635: F) -> (F, F, F, F, F) {
    let t41811 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11506 * t39015;
    let t41814 = t3275 * t38739 * t2867 / F::cast_from(4.0_f64);
    let t41816 = t11002 * t1115 * t2847;
    let t41818 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3269 * t41816;
    let t41821 = F::cast_from(15.0_f64) / F::cast_from(4.0_f64) * t39197 * t1115 * t39198;
    let t41824 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t3472 * t40635;
    (t41811, t41814, t41818, t41821, t41824)
}
