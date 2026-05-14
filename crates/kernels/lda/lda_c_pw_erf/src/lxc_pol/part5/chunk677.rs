//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 677/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk677<F: Float>(t6531: F, t6565: F, t530: F, t186: F, t185: F, t5215: F, t786: F, t1982: F, t808: F, t2100: F, t795: F, t2407: F, t544: F, t494: F, t793: F, t184: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6566 = t6531 + t6565;
    let t6567 = t530 * t6566;
    let t6568 = t186 * t6567;
    let t6570 = 2.0 / 15.0 * t185 * t6568;
    let t6572 = 8.0 / 15.0 * t5215 * t786;
    let t6574 = 4.0 / 15.0 * t1982 * t808;
    let t6576 = 4.0 / 15.0 * t795 * t2100;
    let t6578 = 4.0 / 15.0 * t2407 * t544;
    let t6579 = t494 * t793;
    let t6580 = t6579 * t184;
    (t6566, t6567, t6568, t6570, t6572, t6574, t6576, t6578, t6579, t6580)
}
