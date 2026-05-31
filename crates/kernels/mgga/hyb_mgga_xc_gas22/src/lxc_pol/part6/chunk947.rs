//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 947/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk947<F: Float>(t7: F, t1793: F, t545: F, t1796: F, t1808: F, t3302: F, t3305: F, t461: F, t776: F, t8632: F, t8635: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t8636 = t1793 * t545;
    let t8646 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t8632 * t1808 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t8635 * t8636 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3302 * t1796 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t776 * t1793 + F::cast_from(2.0_f64) * t3305 * t461);
    let t8648 = t222 * t37 * t8646;
    (t8636, t8646, t8648)
}
