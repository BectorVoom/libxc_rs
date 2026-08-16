//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1191/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1191(t1972: f64, t5448: f64, t5451: f64, t5454: f64, t5458: f64, t6268: f64, t1920: f64, t5305: f64, t5464: f64, t15703: f64, t15705: f64, t15707: f64, t15709: f64, t15711: f64, t15713: f64, t15715: f64, t15717: f64, t15719: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15721 = 8.0_f64 / 45.0_f64 * t1972 * t5448;
    let t15723 = 4.0_f64 / 45.0_f64 * t1972 * t5451;
    let t15725 = 4.0_f64 / 9.0_f64 * t1972 * t5454;
    let t15727 = 16.0_f64 / 45.0_f64 * t6268 * t5458;
    let t15729 = 4.0_f64 / 27.0_f64 * t5305 * t1920;
    let t15731 = 4.0_f64 / 27.0_f64 * t1972 * t5464;
    let t15732 = -t15703 - t15705 - t15707 - t15709 + t15711 + t15713 + t15715 + t15717 - t15719 - t15721 - t15723 - t15725 + t15727 + t15729 + t15731;
    (t15721, t15723, t15725, t15727, t15729, t15731, t15732)
}
