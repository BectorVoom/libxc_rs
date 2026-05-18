//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 614/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk614<F: Float>(t1755: F, t4415: F, t1746: F, t1769: F, t1904: F, t462: F, t159: F, t285: F, t1896: F, t477: F, t440: F, t756: F) -> (F, F, F, F, F, F) {
    let t4416 = t4415 * t1755;
    let t4418 = t1769 * t1746;
    let t4422 = t462 * t1904;
    let t4425 = F::new(0.0005811348303577384) * t4422 * t159 * t285;
    let t4427 = t1896 * t477 * t285;
    let t4429 = t756 * t440;
    (t4416, t4418, t4422, t4425, t4427, t4429)
}
