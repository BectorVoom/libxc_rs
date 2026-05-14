//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 762/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk762<F: Float>(t304: F, t4920: F, t1153: F, t2429: F, t3392: F, t3394: F, t3397: F, t368: F, t5130: F, t5133: F, t5135: F, t5139: F, t5143: F, t5147: F, t5151: F, t5155: F, t86: F) -> (F, F) {
    let t5158 = t304 * t4920;
    let t5162 = t3392 - 0.17687407407407407407e-1 * t3394 - 0.26531111111111111111e-1 * t3397 - 0.17687407407407407407e-1 * t5130 - 0.44218518518518518518e-1 * t5133 * t5135 - 0.26531111111111111111e-1 * t1153 * t5139 + 0.53062222222222222222e-1 * t5133 * t5143 - 0.53062222222222222222e-1 * t2429 * t5147 - 0.26531111111111111111e-1 * t5151 - 0.26531111111111111111e-1 * t1153 * t5155 - 0.39796666666666666666e-1 * t86 * t368 * t5158;
    (t5158, t5162)
}
