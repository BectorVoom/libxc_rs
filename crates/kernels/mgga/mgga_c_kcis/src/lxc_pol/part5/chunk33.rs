//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 33/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk33<F: Float>(t60: F, t6: F, t69: F, t63: F, t66: F, t21: F, t2: F) -> (F, F, F, F, F, F) {
    let t70 = F::cast_from(0.0_f64) < t60;
    let t72 = piecewise3::<F>(t70, t60, -t60);
    let t73 = F::cast_from(1.0_f64) / t72;
    let t74 = t6 * t73;
    let t75 = t69 * t74;
    let t78 = F::cast_from(1.0_f64) + t63 * t66 * t75 / F::cast_from(96.0_f64);
    let t79 = F::ln(t78);
    let t81 = F::cast_from(1.0_f64) + F::cast_from(0.66725e-1_f64) * t79;
    let t82 = F::cast_from(1.0_f64) / t81;
    let t84 = F::cast_from(1.0_f64) / t21;
    let t85 = t2 * t84;
    (t72, t75, t78, t81, t82, t85)
}
