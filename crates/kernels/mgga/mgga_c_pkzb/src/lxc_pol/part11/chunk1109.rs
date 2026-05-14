//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1109/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1109<F: Float>(t10780: F, t5734: F, t1855: F, t2783: F, t3550: F, t1084: F, t9389: F, t10783: F, t5771: F, t1083: F, t1899: F, t25873: F, t2782: F, t9228: F, t25908: F, t2751: F) -> (F, F, F, F, F, F, F) {
    let t30205 = 6.0 * t5734 * t10780;
    let t30208 = 6.0 * t1855 * t2783 * t3550;
    let t30211 = 6.0 * t1855 * t1084 * t9389;
    let t30213 = 0.48245938496077605201e2 * t5771 * t10783;
    let t30216 = 0.48245938496077605201e2 * t1899 * t25873 * t1083;
    let t30219 = 0.48245938496077605201e2 * t1899 * t9228 * t2782;
    let t30221 = 6.0 * t25908 * t2751;
    (t30205, t30208, t30211, t30213, t30216, t30219, t30221)
}
