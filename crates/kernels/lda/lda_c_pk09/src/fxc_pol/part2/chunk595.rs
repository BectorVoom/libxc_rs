//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 595/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk595<F: Float>(t151: F, t2983: F, t192: F, t3557: F, t179: F, t205: F, t3553: F, t200: F, t2971: F, t830: F, t3194: F, t2974: F) -> (F, F, F, F, F, F, F, F) {
    let t4684 = t151 * t2983;
    let t4689 = F::new(2.460083242092564) * t192 * t3557;
    let t4692 = F::new(20.705842241814405) * t179 * t3557;
    let t4694 = F::new(3.6857207583175526) * t205 * t3553;
    let t4702 = F::new(2.6972402168825864) * t200 * t3557;
    let t4705 = t830 * t2971;
    let t4706 = t4705 * t3194;
    let t4708 = t4705 * t2974;
    (t4684, t4689, t4692, t4694, t4702, t4705, t4706, t4708)
}
