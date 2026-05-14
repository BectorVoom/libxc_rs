//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 547/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk547<F: Float>(t179: F, t3557: F, t205: F, t3553: F, t200: F, t2971: F, t830: F, t3194: F, t2974: F, t823: F, t825: F, t609: F, t121: F, t4037: F, t340: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4692 = 20.705842241814405 * t179 * t3557;
    let t4694 = 3.6857207583175526 * t205 * t3553;
    let t4702 = 2.6972402168825864 * t200 * t3557;
    let t4705 = t830 * t2971;
    let t4706 = t4705 * t3194;
    let t4708 = t4705 * t2974;
    let t4710 = t823 * t825;
    let t4711 = t4710 * t609;
    let t4712 = t121 * t4711;
    let t4713 = t4037 * t4712;
    let t4715 = t89 * t340;
    (t4692, t4694, t4702, t4705, t4706, t4708, t4710, t4713, t4715)
}
