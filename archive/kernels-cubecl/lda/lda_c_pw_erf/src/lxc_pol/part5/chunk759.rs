//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 759/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk759<F: Float>(t549: F, t6963: F, t1466: F, t1318: F, t2466: F, t3667: F, t593: F, t571: F, t2065: F, t2161: F, t2168: F, t4738: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6964 = t6963 * t549;
    let t6965 = t1466 * t6964;
    let t6967 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t6965;
    let t6968 = t3667 * t2466;
    let t6969 = t6968 * t593;
    let t6970 = t1466 * t6969;
    let t6972 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t571 * t6970;
    let t6973 = t2161 * t2065;
    let t6974 = t1466 * t6973;
    let t6976 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t571 * t6974;
    let t6978 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4738 * t2168;
    (t6964, t6965, t6967, t6968, t6969, t6970, t6972, t6973, t6974, t6976, t6978)
}
