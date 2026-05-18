//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 185/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk185<F: Float>(t348: F, t504: F, t503: F, t11: F, t502: F, t173: F, t184: F) -> (F, F, F, F, F, F) {
    let t505 = t504 * t348;
    let t506 = t503 * t505;
    let t507 = t11 * t506;
    let t509 = t502 + F::new(0.0018891666666666666) * t507;
    let t510 = t173 * t509;
    let t511 = t510 * t184;
    (t505, t506, t507, t509, t510, t511)
}
