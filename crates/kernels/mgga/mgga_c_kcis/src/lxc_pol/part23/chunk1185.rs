//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1185/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1185<F: Float>(t12147: F, t27352: F, t7908: F, t4277: F, t491: F, t16937: F, t27434: F, t16941: F, t27439: F, t11418: F, t1386: F, t1466: F) -> (F, F, F, F, F, F) {
    let t94353 = t7908 * t12147 * t27352;
    let t94393 = t4277 * t491;
    let t94398 = t7908 * t16937 * t27434;
    let t94402 = t7908 * t16941 * t27439;
    let t94408 = t1386 * t11418;
    let t94424 = t1466 * t491;
    (t94353, t94393, t94398, t94402, t94408, t94424)
}
