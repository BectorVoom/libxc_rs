//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1196/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1196<F: Float>(t1313: F, t348: F, t504: F, t5127: F, t519: F, t11983: F, t1318: F, t1403: F, t549: F, t833: F, t4039: F, t795: F) -> (F, F, F) {
    let t14083 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t1313 * t5127 * t504 * t348;
    let t14088 = F::cast_from(24.0_f64) / F::cast_from(5.0_f64) * t1318 * t11983 * t833 * t1403 * t549;
    let t14089 = t795 * t4039;
    (t14083, t14088, t14089)
}
