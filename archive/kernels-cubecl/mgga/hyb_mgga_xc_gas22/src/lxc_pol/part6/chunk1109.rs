//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1109/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1109<F: Float>(t132: F, t3925: F, t6975: F, t2460: F, t3938: F, t10325: F, t1794: F, t3463: F, t675: F, t937: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t10900 = t6975 * t3925;
    let t10905 = t2460 * t3938;
    let t10911 = piecewise3::<F>(t133, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10900 * t675 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3463 * t1794 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10905 * t675 - t937 * t10325 / F::cast_from(3.0_f64));
    let t10913 = t222 * t37 * t10911;
    (t10900, t10905, t10911, t10913)
}
