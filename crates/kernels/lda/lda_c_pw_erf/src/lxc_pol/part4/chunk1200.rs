//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1200/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1200<F: Float>(t16456: F, t4776: F, t571: F, t14257: F, t16461: F, t1472: F, t6389: F, t12299: F, t2027: F, t4738: F, t4826: F, t4831: F, t1325: F, t1440: F, t6997: F, t944: F) -> (F, F, F, F, F, F, F) {
    let t17736 = 32.0 / 81.0 * t571 * t4776 * t16456;
    let t17739 = 352.0 / 243.0 * t571 * t14257 * t16461;
    let t17741 = 32.0 / 27.0 * t1472 * t6389;
    let t17743 = 32.0 / 45.0 * t12299 * t2027;
    let t17745 = 16.0 / 45.0 * t4738 * t4826;
    let t17747 = 64.0 / 45.0 * t4738 * t4831;
    let t17751 = 4.0 / 15.0 * t1325 * t1440 * t6997 * t944;
    (t17736, t17739, t17741, t17743, t17745, t17747, t17751)
}
