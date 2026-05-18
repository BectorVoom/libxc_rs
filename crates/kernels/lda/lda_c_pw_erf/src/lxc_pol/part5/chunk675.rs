//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 675/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk675<F: Float>(t43: F, t1781: F, t348: F, t47: F, t5982: F, t5987: F, t5992: F, t943: F, t2334: F, t2966: F, t2337: F, t950: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t5996 = piecewise3::<f64>(t44, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5982 * t348 + F::new(16.0) / F::new(9.0) * t1781 * t943 + F::new(4.0) / F::new(9.0) * t5987 * t348 + F::new(4.0) / F::new(3.0) * t47 * t5992);
    let t5997 = t2966 * t2334;
    let t6002 = t950 * t2337;
    let t6005 = -t5992;
    (t5996, t5997, t6002, t6005)
}
