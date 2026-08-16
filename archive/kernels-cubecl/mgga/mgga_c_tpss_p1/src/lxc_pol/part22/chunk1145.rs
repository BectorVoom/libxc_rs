//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1145/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1145<F: Float>(t30: F, t12767: F, t737: F, t9969: F, t12727: F, t187: F, t10016: F, t10022: F, t1288: F, t9924: F, t2: F, t3217: F, t1197: F, t12700: F, t1991: F, t22: F, t3218: F, t4380: F, t4383: F, t555: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t12769 = F::cast_from(0.11696447245269292414e1_f64) * t12767 * t737;
    let t12770 = F::cast_from(0.18311447306006545054e-3_f64) * t9969;
    let t12775 = F::cast_from(0.19751673498613801407e-1_f64) * t12727 * t187;
    let t12779 = F::cast_from(24.0_f64) * t10016;
    let t12780 = F::cast_from(48.0_f64) * t10022;
    let t12781 = t9924 * t1288;
    let t12784 = t3217 * t2;
    let t12794 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12781 * t3218 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12784 * t12700 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4380 * t1991 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1197 * t555 - F::cast_from(4.0_f64) * t4383 * t22);
    (t12769, t12770, t12775, t12779, t12780, t12794)
}
