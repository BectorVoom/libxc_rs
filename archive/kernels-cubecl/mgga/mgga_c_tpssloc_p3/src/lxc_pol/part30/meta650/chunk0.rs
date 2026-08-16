//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2064/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2064<F: Float>(t23665: F, t25545: F, t25503: F, t10216: F, t381: F, t10474: F, t82514: F, t25483: F, t23384: F, t25456: F, t362: F, t4657: F) -> (F, F, F, F, F, F, F) {
    let t89158 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25545;
    let t89175 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25503;
    let t89176 = t381 * t10216;
    let t89204 = t82514 * t10474 * t381;
    let t89210 = t82514 * t25483;
    let t89224 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25456;
    let t89235 = t362 * t4657;
    (t89158, t89175, t89176, t89204, t89210, t89224, t89235)
}
