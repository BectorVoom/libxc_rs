//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 499/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk499<F: Float>(t2437: F, t503: F, t11: F, t1501: F, t1964: F, t2431: F, t2435: F, t173: F, t184: F) -> (F, F, F, F, F) {
    let t2438 = t503 * t2437;
    let t2439 = t11 * t2438;
    let t2441 = -t1501 - F::new(0.0012594444444444445) * t1964 + F::new(0.0012594444444444445) * t2431 - F::new(0.003778333333333333) * t2435 + F::new(0.0018891666666666666) * t2439;
    let t2442 = t173 * t2441;
    let t2443 = t2442 * t184;
    (t2438, t2439, t2441, t2442, t2443)
}
