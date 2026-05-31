//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1162/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1162<F: Float>(t2100: F, t2407: F, t1284: F, t7838: F, t39: F, t8327: F, t186: F, t220: F, t548: F, t1982: F, t2499: F, t2505: F, t6580: F) -> (F, F, F, F, F, F) {
    let t21294 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2407 * t2100;
    let t21296 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1284 * t7838;
    let t21299 = -F::cast_from(6.0_f64) * t39 - F::cast_from(12.0_f64) * t8327;
    let t21303 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t548 * t186 * t220 * t21299;
    let t21305 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t1982 * t2499;
    let t21307 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t6580 * t2505;
    (t21294, t21296, t21299, t21303, t21305, t21307)
}
