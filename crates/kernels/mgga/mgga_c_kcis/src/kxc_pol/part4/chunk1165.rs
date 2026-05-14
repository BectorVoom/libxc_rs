//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1165/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1165<F: Float>(t16869: F, t16899: F, t16936: F, t16989: F, t1386: F, t2038: F, t3801: F, t4162: F, t4160: F, t3955: F, t5748: F, t1464: F, t12263: F, t12271: F, t12275: F, t12277: F, t12279: F, t12286: F, t12303: F, t12306: F, t12307: F, t1364: F, t16805: F, t16806: F, t16809: F, t16812: F, t16820: F, t16824: F, t3961: F, t3964: F, t5738: F, t5886: F) -> (F, F, F) {
    let t16991 = t16869 + t16899 + t16936 + t16989;
    let t16992 = t16991 * t1386;
    let t16995 = t2038 * t3801;
    let t16996 = t4162 * t16995;
    let t16997 = t4160 * t16996;
    let t16999 = t5748 * t3955;
    let t17000 = t1464 * t16999;
    let t17004 = -t16805 + 0.22109259259259259258e-2 * t16806 - t16809 + 0.99491666666666666664e-2 * t16812 - 0.3684876543209876543e-3 * t12263 + 0.33163888888888888888e-2 * t12271 - 0.73697530864197530861e-3 * t12275 + 0.11054629629629629629e-2 * t12277 + 0.11054629629629629629e-2 * t12279 - 0.11054629629629629629e-2 * t12303 - 0.55273148148148148147e-3 * t16820 + t12306 + 0.16581944444444444444e-2 * t12307 - 0.2671335375e-1 * t3961 * t16824 + 0.178089025e-1 * t12286 * t5886 - 0.66725e-1 * t1364 * t16992 + 0.33163888888888888888e-2 * t16997 - 0.24872916666666666666e-2 * t17000 - 0.13345e0 * t3964 * t5738;
    (t16997, t17000, t17004)
}
