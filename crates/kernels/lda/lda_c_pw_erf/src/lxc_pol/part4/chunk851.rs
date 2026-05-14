//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 851/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk851<F: Float>(t1326: F, t6492: F, t519: F, t1243: F, t6352: F, t11: F, t3536: F, t6418: F, t6422: F, t1953: F, t538: F, t6331: F, t1268: F, t6336: F, t3516: F, t6442: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6493 = t1326 * t6492;
    let t6495 = 8.0 / 15.0 * t519 * t6493;
    let t6501 = t1243 * t6352;
    let t6502 = t11 * t6501;
    let t6504 = t3536 * t6418;
    let t6505 = t11 * t6504;
    let t6507 = t1243 * t6422;
    let t6508 = t1953 * t6507;
    let t6510 = t538 * t6331;
    let t6513 = t1268 * t6336;
    let t6516 = t1268 * t6352;
    let t6519 = t3516 * t6418;
    let t6522 = t1268 * t6422;
    let t6525 = t538 * t6492;
    let t6528 = t538 * t6442;
    (t6493, t6495, t6501, t6502, t6504, t6505, t6507, t6508, t6510, t6513, t6516, t6519, t6522, t6525, t6528)
}
