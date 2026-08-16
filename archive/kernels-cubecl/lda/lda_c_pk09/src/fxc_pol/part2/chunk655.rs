//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 655/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk655<F: Float>(t1303: F, t5785: F, t326: F, t741: F, t623: F, t93: F, t1336: F, t1561: F, t1625: F, t1593: F, t327: F, t5420: F) -> (F, F, F, F, F, F) {
    let t5786 = t1303 * t5785;
    let t5794 = t741 * t326;
    let t5795 = t5794 * t623;
    let t5796 = t93 * t5795;
    let t5799 = t1561 * t1336;
    let t5800 = t5799 * t1625;
    let t5802 = t1593 * t1336;
    let t5803 = t5802 * t1625;
    let t5805 = t327 * t5420;
    (t5786, t5794, t5796, t5800, t5803, t5805)
}
