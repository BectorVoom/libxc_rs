//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1175/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1175<F: Float>(t20711: F, t549: F, t3974: F, t4515: F, t3965: F, t4479: F, t6488: F, t17123: F, t17156: F, t6631: F, t808: F, t9934: F) -> (F, F, F, F, F, F, F) {
    let t21420 = t20711 * t549;
    let t21423 = F::new(16.0) / F::new(15.0) * t3974 * t4515 * t21420;
    let t21426 = F::new(16.0) / F::new(15.0) * t3965 * t4479 * t6488;
    let t21427 = F::new(8.0) / F::new(15.0) * t17123;
    let t21428 = F::new(8.0) / F::new(27.0) * t17156;
    let t21430 = F::new(2.0) / F::new(5.0) * t6631 * t808;
    let t21431 = F::new(16.0) / F::new(405.0) * t9934;
    (t21420, t21423, t21426, t21427, t21428, t21430, t21431)
}
