//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 696/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk696<F: Float>(t4574: F, t558: F, t2070: F, t211: F, t1524: F, t835: F, t2001: F, t3854: F, t1318: F, t3804: F, t3856: F, t3861: F, t3865: F, t1511: F, t793: F, t184: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4575 = t4574 * t558;
    let t4576 = t2070 * t4575;
    let t4578 = 8.0 / 45.0 * t211 * t4576;
    let t4580 = 4.0 / 15.0 * t1524 * t835;
    let t4581 = t3854 * t2001;
    let t4583 = 32.0 / 135.0 * t1318 * t4581;
    let t4584 = 16.0 / 135.0 * t3804;
    let t4585 = 32.0 / 135.0 * t3856;
    let t4586 = 32.0 / 135.0 * t3861;
    let t4587 = 16.0 / 135.0 * t3865;
    let t4588 = t1511 * t793;
    let t4589 = t4588 * t184;
    (t4575, t4576, t4578, t4580, t4581, t4583, t4584, t4585, t4586, t4587, t4588, t4589)
}
