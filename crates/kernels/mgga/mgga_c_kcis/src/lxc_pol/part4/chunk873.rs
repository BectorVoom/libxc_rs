//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 873/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk873<F: Float>(t706: F, t9220: F, t2385: F, t684: F, t2379: F, t687: F, t2390: F, t688: F, t707: F, t8533: F, t8541: F, t8753: F, t8757: F, t8924: F, t8926: F, t8932: F, t8934: F, t8937: F) -> (F,) {
    let t9221 = t9220 * t706;
    let t9229 = t684 * t2385;
    let t9232 = t2379 * t687;
    let t9235 = 0.59694999999999999999e-1 * t8533 - 0.59694999999999999999e-1 * t8541 - 0.99491666666666666664e-2 * t8753 - 0.29847499999999999999e-1 * t8757 - 0.66725e-1 * t688 * t9221 + 0.79593333333333333331e-1 * t8924 + 0.39796666666666666665e-1 * t8926 - 0.92858888888888888885e-1 * t8932 - 0.79593333333333333331e-1 * t8934 - 0.29847499999999999999e-1 * t8937 + 0.2671335375e-1 * t9229 * t2390 - 0.200175e0 * t9232 * t707;
    (t9235,)
}
