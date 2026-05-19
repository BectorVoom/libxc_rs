//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1013/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1013<F: Float>(t3476: F, t521: F, t11857: F, t11866: F, t1458: F, t3518: F, t1245: F, t537: F, t188: F, t11829: F, t11832: F, t11834: F, t11837: F, t11846: F, t11848: F, t11851: F, t11854: F, t11855: F, t11861: F, t1268: F, t2061: F, t3411: F, t3481: F, t538: F, t9847: F, t9866: F, t9868: F, t9891: F, t9893: F) -> (F, F, F, F) {
    let t11867 = t521 * t3476;
    let t11869 = t11866 * t11867 * t11857;
    let t11871 = t1458 * t3518;
    let t11873 = t11866 * t11871 * t11857;
    let t11875 = t537 * t1245;
    let t11879 = t188 * t1245;
    let t11881 = t11866 * t11879 * t11857;
    let t11885 = F::cast_from(0.11197407407407407_f64) * t9847 + F::cast_from(0.09597777777777777_f64) * t9866 + F::cast_from(0.07198333333333333_f64) * t9868 + F::cast_from(0.019753086419753086_f64) * t11829 + F::new(0.4319) * t11832 + F::cast_from(0.03732469135802469_f64) * t11834 - F::cast_from(0.14396666666666666_f64) * t11837 - F::cast_from(0.013333333333333334_f64) * t2061 * t1268 * t3481 + F::new(0.08) * t2061 * t538 * t3411 - F::cast_from(0.28444444444444444_f64) * t11846 - F::cast_from(1.1757277777777777_f64) * t11848 + F::cast_from(0.10666666666666667_f64) * t11851 + F::new(0.04) * t11854 * t11855 * t11857 - F::cast_from(0.008888888888888889_f64) * t11854 * t11861 * t11857 + F::new(0.4319) * t11869 - F::cast_from(0.11997222222222222_f64) * t11873 - F::new(0.12) * t11854 * t11875 * t11857 - F::new(0.64785) * t11881 + F::cast_from(0.044444444444444446_f64) * t9891 + F::cast_from(0.02666666666666667_f64) * t9893;
    (t11869, t11873, t11881, t11885)
}
