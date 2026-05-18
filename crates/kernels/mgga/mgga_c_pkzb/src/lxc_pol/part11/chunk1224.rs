//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1224/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1224<F: Float>(t1083: F, t1899: F, t25873: F, t2782: F, t9228: F, t25908: F, t2751: F, t25832: F, t2787: F, t7483: F, t9225: F, t7411: F, t9229: F) -> (F, F, F, F, F, F) {
    let t30216 = F::new(0.48245938496077605201e2) * t1899 * t25873 * t1083;
    let t30219 = F::new(0.48245938496077605201e2) * t1899 * t9228 * t2782;
    let t30221 = F::new(6.0) * t25908 * t2751;
    let t30223 = F::new(0.48245938496077605201e2) * t25832 * t2787;
    let t30225 = F::new(6.0) * t7483 * t9225;
    let t30227 = F::new(0.48245938496077605201e2) * t7411 * t9229;
    (t30216, t30219, t30221, t30223, t30225, t30227)
}
