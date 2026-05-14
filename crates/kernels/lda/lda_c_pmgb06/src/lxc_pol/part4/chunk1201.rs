//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1201/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1201<F: Float>(t15703: F, t15705: F, t15707: F, t15709: F, t15711: F, t15713: F, t15715: F, t15717: F, t15719: F, t15721: F, t15723: F, t15725: F, t9342: F, t9345: F, t9348: F, t15727: F, t15729: F, t15731: F, t15734: F, t15736: F, t15738: F, t15740: F, t15743: F, t15745: F, t15746: F, t15747: F, t15748: F, t15753: F, t15754: F, t15758: F) -> (F, F) {
    let t18176 = -0.13298177777777778 * t9342 - t9345 + t9348 - t15703 - t15705 - t15707 - t15709 + t15711 + t15713 + t15715 + t15717 - t15719 - t15721 - t15723 - t15725;
    let t18177 = t15727 + t15729 + t15731 + t15734 + t15736 - t15738 + t15740 - t15743 - t15745 - t15746 + t15747 - t15748 - t15753 + t15754 - t15758;
    (t18176, t18177)
}
