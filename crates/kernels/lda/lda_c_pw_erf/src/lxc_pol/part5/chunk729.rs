//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 729/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk729<F: Float>(t6531: F, t6565: F, t530: F, t186: F, t185: F, t5215: F, t786: F, t1982: F, t808: F, t2100: F, t795: F, t2407: F, t544: F) -> (F, F, F, F, F, F, F, F) {
    let t6566 = t6531 + t6565;
    let t6567 = t530 * t6566;
    let t6568 = t186 * t6567;
    let t6570 = F::new(2.0) / F::new(15.0) * t185 * t6568;
    let t6572 = F::new(8.0) / F::new(15.0) * t5215 * t786;
    let t6574 = F::new(4.0) / F::new(15.0) * t1982 * t808;
    let t6576 = F::new(4.0) / F::new(15.0) * t795 * t2100;
    let t6578 = F::new(4.0) / F::new(15.0) * t2407 * t544;
    (t6566, t6567, t6568, t6570, t6572, t6574, t6576, t6578)
}
