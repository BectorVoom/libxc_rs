//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1182/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1182<F: Float>(t12: F, t10627: F, t16425: F, t600: F, t10670: F, t1769: F, t1064: F, t10760: F, t10764: F, t1430: F, t207: F, t2732: F, t2735: F, t28874: F, t28877: F, t28885: F, t3510: F, t439: F, t8729: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t29024 = t10627 * t16425 * t600;
    let t29032 = t1769 * t10670;
    let t29049 = piecewise3::<F>(t84, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t10760 * t439 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3510 * t1430 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2732 * t28874 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2735 * t28877 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1064 * t8729 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10764 * t439 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t207 * t28885);
    (t29024, t29032, t29049)
}
