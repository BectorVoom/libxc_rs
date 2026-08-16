//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 505/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk505<F: Float>(t2104: F, t813: F, t1284: F, t544: F, t795: F, t511: F, t808: F, t181: F, t494: F, t184: F) -> (F, F, F, F, F, F) {
    let t2106 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2104 * t813;
    let t2108 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1284 * t813;
    let t2110 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t795 * t544;
    let t2112 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t511 * t808;
    let t2113 = t494 * t181;
    let t2114 = t2113 * t184;
    (t2106, t2108, t2110, t2112, t2113, t2114)
}
