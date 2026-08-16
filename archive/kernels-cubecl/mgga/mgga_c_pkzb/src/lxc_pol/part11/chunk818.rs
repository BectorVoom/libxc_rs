//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 818/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk818<F: Float>(t12: F, t1430: F, t2540: F, t439: F, t87: F, t8721: F, t8726: F, t8729: F, t3371: F, t5106: F, t1651: F, t3374: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t8733 = piecewise3::<F>(t84, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8721 * t439 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2540 * t1430 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8726 * t439 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t87 * t8729);
    let t8734 = t5106 * t3371;
    let t8739 = t1651 * t3374;
    let t8742 = -t8729;
    (t8733, t8734, t8739, t8742)
}
