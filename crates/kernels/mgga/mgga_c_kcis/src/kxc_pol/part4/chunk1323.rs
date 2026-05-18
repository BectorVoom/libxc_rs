//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1323/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1323<F: Float>(t16869: F, t16899: F, t16936: F, t16989: F, t1386: F, t2038: F, t3801: F, t4162: F, t4160: F, t3955: F, t5748: F, t1464: F) -> (F, F, F) {
    let t16991 = t16869 + t16899 + t16936 + t16989;
    let t16992 = t16991 * t1386;
    let t16995 = t2038 * t3801;
    let t16996 = t4162 * t16995;
    let t16997 = t4160 * t16996;
    let t16999 = t5748 * t3955;
    let t17000 = t1464 * t16999;
    (t16992, t16997, t17000)
}
