//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1022/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1022<F: Float>(t1287: F, t1318: F, t5269: F, t593: F, t833: F, t1381: F, t5270: F, t1466: F, t3667: F, t571: F, t1401: F, t3899: F) -> (F, F, F, F, F) {
    let t11978 = F::new(8.0) / F::new(5.0) * t1318 * t5269 * t833 * t1287 * t593;
    let t11982 = F::new(8.0) / F::new(5.0) * t1318 * t5269 * t5270 * t1381;
    let t11983 = t1466 * t3667;
    let t11984 = t833 * t593;
    let t11988 = F::new(12.0) / F::new(5.0) * t571 * t11983 * t11984 * t1381;
    let t11989 = t3899 * t1401;
    (t11978, t11982, t11983, t11988, t11989)
}
