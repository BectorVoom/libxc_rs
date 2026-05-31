//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 632/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk632<F: Float>(t3727: F, t577: F, t1390: F, t1392: F, t494: F, t1440: F, t1325: F, t1340: F, t1449: F, t519: F, t1460: F, t2954: F) -> (F, F, F, F, F, F, F, F) {
    let t3729 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t3727 * t577;
    let t3731 = t1390 * t494 * t1392;
    let t3732 = t1440 * t3731;
    let t3734 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1325 * t3732;
    let t3735 = t1449 * t1340;
    let t3736 = t519 * t3735;
    let t3737 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3736;
    let t3738 = t1460 * t2954;
    (t3729, t3731, t3732, t3734, t3735, t3736, t3737, t3738)
}
