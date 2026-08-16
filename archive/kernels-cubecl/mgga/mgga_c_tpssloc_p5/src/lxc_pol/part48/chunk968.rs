//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 968/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk968<F: Float>(t23204: F, t31419: F, t6562: F, t112946: F, t112949: F, t113038: F, t113041: F, t113045: F, t114937: F, t114939: F, t114944: F, t114945: F, t114960: F, t22974: F, t23191: F, t24325: F, t25168: F, t259: F, t26728: F, t2718: F, t2720: F, t2742: F, t31361: F, t31423: F, t6627: F, t7087: F, t798: F, t855: F, t8562: F) -> F {
    let t114965 = t6562 * t23204 * t31419;
    let t114967 = -F::cast_from(0.82246703342411321825e-2_f64) * t114937 + F::cast_from(0.38381794893125283518e-1_f64) * t114939 - t7087 * t23191 + t114944 + t112946 + t112949 + F::cast_from(0.38381794893125283518e-1_f64) * t114945 + t113038 + F::cast_from(2.0_f64) * t798 * t31361 * t259 + t113041 - t113045 + F::cast_from(2.0_f64) * t855 * t2718 * t8562 * t2742 - F::cast_from(6.0_f64) * t25168 * t26728 * t22974 + F::cast_from(4.0_f64) * t6627 * t24325 - F::cast_from(0.16449340668482264365e-1_f64) * t114960 + F::cast_from(2.0_f64) * t31423 * t2720 + F::cast_from(0.82246703342411321824e-2_f64) * t114965;
    t114967
}
