//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1146/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1146<F: Float>(t132: F, t137: F, t2106: F, t6225: F, t464: F, t7501: F, t477: F, t2489: F, t5305: F, t10720: F, t10727: F, t17547: F, t17550: F, t20759: F, t20762: F, t20764: F, t20767: F, t20768: F) -> (F, F, F, F) {
    let t20773 = t132 * t137 * t2106 * t6225 / F::new(10.0);
    let t20774 = t7501 * t464;
    let t20778 = t132 * t137 * t20774 * t477 / F::new(30.0);
    let t20780 = F::new(2.0) / F::new(15.0) * t5305 * t2489;
    let t20781 = F::cast_from(0.6492624817418906_f64) * t17547 + F::cast_from(0.03354522822333102_f64) * t17550 + t20759 - t20762 + t20764 + t20767 - t20768 + F::new(4.0) / F::new(3.0) * t10720 + t10727 - t20773 - t20778 - t20780;
    (t20773, t20778, t20780, t20781)
}
