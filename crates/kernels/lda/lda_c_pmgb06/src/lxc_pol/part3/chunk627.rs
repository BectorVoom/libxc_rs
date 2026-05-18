//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 627/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk627<F: Float>(t3588: F, t3615: F, t3569: F, t3573: F, t3578: F, t3580: F, t3583: F, t3586: F, t3590: F, t3597: F, t360: F, t3602: F, t3604: F, t3607: F, t3608: F, t3613: F, t63: F) -> (F, F) {
    let t3616 = t3615 * t3588;
    let t3619 = -F::new(1.46904) * t3569 + F::new(2.20356) * t3573 + t3578 + t3580 - F::new(2.93808) * t3583 - F::new(3.0) / F::new(2.0) * t3586 - F::new(6.0) * t360 * t3590 - F::new(8.81424) * t3597 - t3602 - t3604 - t3607 - F::new(1.46904) * t63 * t3608 - t3613 - F::new(29.3808) * t63 * t3616;
    (t3616, t3619)
}
