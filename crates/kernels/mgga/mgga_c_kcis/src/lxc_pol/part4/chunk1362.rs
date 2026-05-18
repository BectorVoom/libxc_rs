//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1362/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1362<F: Float>(t12520: F, t492: F, t15973: F, t6028: F, t2051: F, t4307: F, t16751: F, t577: F, t1548: F, t16622: F, t4288: F, t2042: F, t4269: F) -> (F, F, F, F, F) {
    let t17508 = t12520 * t492;
    let t17509 = t6028 * t15973;
    let t17510 = t17508 * t17509;
    let t17512 = t2051 * t4307;
    let t17514 = t16751 * t577;
    let t17515 = t17514 * t1548;
    let t17517 = t16622 * t577;
    let t17518 = t17517 * t4288;
    let t17520 = t2042 * t4269;
    (t17510, t17512, t17515, t17518, t17520)
}
