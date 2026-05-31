//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1192/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1192<F: Float>(t12: F, t2735: F, t500: F, t1429: F, t17361: F, t1837: F, t19633: F, t19636: F, t19642: F, t19645: F, t2732: F, t439: F, t5094: F, t5100: F, t5528: F, t652: F, t7337: F, t7340: F, t8: F, t82: F, t972: F, zeta_threshold: F) -> F {
    let t84 = t12 <= zeta_threshold;
    let t20741 = F::cast_from(8.0_f64) * t2735 * t500;
    let t20743 = piecewise3::<F>(t84, F::cast_from(0.0_f64), F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t17361 * t972 * t5094 - F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t5528 * t8 * t19633 - F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t7337 * t19636 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1837 * t1429 * t439 - F::cast_from(8.0_f64) * t7340 * t19642 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t7340 * t19645 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2732 * t5100 + F::cast_from(4.0_f64) * t652 * t82 - t20741);
    t20743
}
