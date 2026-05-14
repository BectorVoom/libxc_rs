//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 481/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk481<F: Float>(t2419: F, t557: F, t11: F, t1491: F, t1941: F, t2413: F, t2417: F, t203: F, t184: F) -> (F, F, F, F, F) {
    let t2420 = t557 * t2419;
    let t2421 = t11 * t2420;
    let t2423 = -t1491 - 0.0012594444444444445 * t1941 + 0.0012594444444444445 * t2413 - 0.003778333333333333 * t2417 + 0.0018891666666666666 * t2421;
    let t2424 = t203 * t2423;
    let t2425 = t2424 * t184;
    (t2420, t2421, t2423, t2424, t2425)
}
