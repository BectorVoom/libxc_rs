//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1191/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1191<F: Float>(t1972: F, t5448: F, t5451: F, t5454: F, t5458: F, t6268: F, t1920: F, t5305: F, t5464: F, t15703: F, t15705: F, t15707: F, t15709: F, t15711: F, t15713: F, t15715: F, t15717: F, t15719: F) -> (F, F, F, F, F, F, F) {
    let t15721 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1972 * t5448;
    let t15723 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1972 * t5451;
    let t15725 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1972 * t5454;
    let t15727 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t6268 * t5458;
    let t15729 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5305 * t1920;
    let t15731 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1972 * t5464;
    let t15732 = -t15703 - t15705 - t15707 - t15709 + t15711 + t15713 + t15715 + t15717 - t15719 - t15721 - t15723 - t15725 + t15727 + t15729 + t15731;
    (t15721, t15723, t15725, t15727, t15729, t15731, t15732)
}
