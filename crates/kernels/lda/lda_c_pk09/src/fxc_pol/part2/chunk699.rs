//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 699/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk699<F: Float>(t6519: F, t6522: F, t6527: F, t6319: F, t6325: F, t6547: F, t6550: F, t6464: F, t1842: F, t6593: F, t525: F, t6601: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6637 = F::new(8.0) * t6519;
    let t6638 = F::new(2.6666666666666665) * t6522;
    let t6639 = F::new(8.0) * t6527;
    let t6642 = F::new(0.505765839233979) * t6319;
    let t6649 = F::new(0.337177226155986) * t6325;
    let t6650 = F::new(0.2222222222222222) * t6547;
    let t6651 = F::new(2.6666666666666665) * t6550;
    let t6655 = F::new(0.112392408718662) * t6464;
    let t6662 = t1842 * t6593;
    let t6665 = F::new(8.282336896725763) * t525 * t6601;
    (t6637, t6638, t6639, t6642, t6649, t6650, t6651, t6655, t6662, t6665)
}
