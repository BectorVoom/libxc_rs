//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 714/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk714<F: Float>(t10056: F, t10058: F, t1953: F, t702: F, t72: F, t2435: F, t5928: F, t1737: F, t699: F, t1364: F, t2448: F, t623: F) -> (F, F, F, F, F, F, F, F) {
    let t10377 = F::new(0.40911992481368012596e-1) * t10056;
    let t10378 = F::new(0.5454932330849068346e-1) * t10058;
    let t10379 = t1953 * t702;
    let t10380 = t72 * t10379;
    let t10381 = t5928 * t2435;
    let t10382 = F::new(0.79828278012425390428e-1) * t10381;
    let t10387 = t699 * t1737;
    let t10388 = t1364 * t10387;
    let t10389 = F::new(0.23948483403727617128e0) * t10388;
    let t10390 = t623 * t2448;
    (t10377, t10378, t10379, t10380, t10382, t10387, t10389, t10390)
}
