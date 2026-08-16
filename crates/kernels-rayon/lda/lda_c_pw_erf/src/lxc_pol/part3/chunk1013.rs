//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1013/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1013(t3476: f64, t521: f64, t11857: f64, t11866: f64, t1458: f64, t3518: f64, t1245: f64, t537: f64, t188: f64, t11829: f64, t11832: f64, t11834: f64, t11837: f64, t11846: f64, t11848: f64, t11851: f64, t11854: f64, t11855: f64, t11861: f64, t1268: f64, t2061: f64, t3411: f64, t3481: f64, t538: f64, t9847: f64, t9866: f64, t9868: f64, t9891: f64, t9893: f64) -> (f64, f64, f64, f64) {
    let t11867 = t521 * t3476;
    let t11869 = t11866 * t11867 * t11857;
    let t11871 = t1458 * t3518;
    let t11873 = t11866 * t11871 * t11857;
    let t11875 = t537 * t1245;
    let t11879 = t188 * t1245;
    let t11881 = t11866 * t11879 * t11857;
    let t11885 = 0.11197407407407407_f64 * t9847 + 0.09597777777777777_f64 * t9866 + 0.07198333333333333_f64 * t9868 + 0.019753086419753086_f64 * t11829 + 0.4319_f64 * t11832 + 0.03732469135802469_f64 * t11834 - 0.14396666666666666_f64 * t11837 - 0.013333333333333334_f64 * t2061 * t1268 * t3481 + 0.08_f64 * t2061 * t538 * t3411 - 0.28444444444444444_f64 * t11846 - 1.1757277777777777_f64 * t11848 + 0.10666666666666667_f64 * t11851 + 0.04_f64 * t11854 * t11855 * t11857 - 0.008888888888888889_f64 * t11854 * t11861 * t11857 + 0.4319_f64 * t11869 - 0.11997222222222222_f64 * t11873 - 0.12_f64 * t11854 * t11875 * t11857 - 0.64785_f64 * t11881 + 0.044444444444444446_f64 * t9891 + 0.02666666666666667_f64 * t9893;
    (t11869, t11873, t11881, t11885)
}
